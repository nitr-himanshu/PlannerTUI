mod app;
mod config;
mod event;
mod model;
mod storage;
mod ui;
mod widget;

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use tokio::sync::mpsc;

use app::App;
use config::Config;
use event::AppEvent;
use storage::{json::JsonProvider, DataProvider};

fn data_dir() -> Result<std::path::PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("cannot determine executable directory"))?;
    Ok(dir.join(".planner_tui"))
}

fn ensure_data_dir() -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    let data_dir = data_dir()?;
    std::fs::create_dir_all(&data_dir)?;

    let config_path = data_dir.join("config.json");
    let items_path = data_dir.join("items.json");

    if !config_path.exists() {
        let default = config::defaults::default_config();
        default.save(&config_path)?;
    }

    if !items_path.exists() {
        let default = config::defaults::default_items();
        let provider = JsonProvider { path: items_path.clone() };
        provider.save(&default)?;
    }

    Ok((config_path, items_path))
}

async fn run(config: Config, items: storage::Items) -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(config, items);

    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);

    tokio::task::spawn_blocking(move || loop {
        if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false) {
            match crossterm::event::read() {
                Ok(Event::Key(key)) => {
                    if tx.blocking_send(AppEvent::Key(key)).is_err() {
                        break;
                    }
                }
                Ok(Event::Mouse(mouse)) => {
                    if tx.blocking_send(AppEvent::Mouse(mouse)).is_err() {
                        break;
                    }
                }
                _ => {}
            }
        }
    });

    let mut tick = tokio::time::interval(Duration::from_secs(1));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let mut panel_layout: Vec<(usize, Rect)> = Vec::new();

    loop {
        if let Ok((cols, rows)) = crossterm::terminal::size() {
            let main_area = Rect::new(0, 0, cols, rows.saturating_sub(1));
            if let Ok(resolved) = ui::grid::resolve(&app.config, main_area) {
                panel_layout = resolved.iter().enumerate().map(|(i, p)| (i, p.rect)).collect();
            }
        }

        terminal.draw(|frame| ui::render(frame, &app))?;

        tokio::select! {
            _ = tick.tick() => {
                app.on_tick();
            }
            Some(ev) = rx.recv() => {
                match ev {
                    AppEvent::Key(key) => {
                        if event::keyboard::handle(key, &mut app) {
                            break;
                        }
                    }
                    AppEvent::Mouse(mouse) => {
                        event::mouse::handle(mouse, &mut app, &panel_layout);
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let (config_path, items_path) = ensure_data_dir()?;
    let config = Config::load(&config_path)?;
    let provider = JsonProvider { path: items_path };
    let items = provider.load()?;

    if let Err(e) = run(config, items).await {
        terminal::disable_raw_mode().ok();
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).ok();
        return Err(e);
    }

    Ok(())
}
