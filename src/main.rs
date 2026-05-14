mod app;
mod config;
mod event;
mod model;
mod setup;
mod storage;
mod ui;
mod widget;

use std::io;
use std::path::PathBuf;
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

fn data_dir() -> Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("cannot determine executable directory"))?;
    Ok(dir.join(".planner_tui"))
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(io::stdout()))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
    terminal::disable_raw_mode().ok();
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).ok();
    terminal.show_cursor().ok();
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    config: Config,
    items: storage::Items,
    items_path: PathBuf,
) -> Result<()> {
    let mut app = App::new(config, items, items_path);

    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);

    tokio::task::spawn_blocking(move || loop {
        if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false) {
            match crossterm::event::read() {
                Ok(Event::Key(key)) if tx.blocking_send(AppEvent::Key(key)).is_err() => break,
                Ok(Event::Mouse(mouse)) if tx.blocking_send(AppEvent::Mouse(mouse)).is_err() => break,
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let data_dir = data_dir()?;
    let is_first_run = !data_dir.exists();

    let mut terminal = init_terminal()?;

    let result = run_all(&mut terminal, data_dir, is_first_run).await;

    restore_terminal(&mut terminal);
    result
}

async fn run_all(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    data_dir: PathBuf,
    is_first_run: bool,
) -> Result<()> {
    let config_path = data_dir.join("config.json");
    let items_path = data_dir.join("items.json");

    let (config, items) = if is_first_run {
        std::fs::create_dir_all(&data_dir)?;

        let result = setup::run(terminal)?;

        result.config.save(&config_path)?;
        JsonProvider { path: items_path.clone() }.save(&result.items)?;

        (result.config, result.items)
    } else {
        if !config_path.exists() {
            config::defaults::default_config().save(&config_path)?;
        }
        if !items_path.exists() {
            JsonProvider { path: items_path.clone() }
                .save(&config::defaults::default_items())?;
        }
        (
            Config::load(&config_path)?,
            JsonProvider { path: items_path.clone() }.load()?,
        )
    };

    run_app(terminal, config, items, items_path).await
}
