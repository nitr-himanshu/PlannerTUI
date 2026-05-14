mod app;
mod config;
mod model;
mod storage;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::App;
use config::Config;
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

fn run(config: Config, items: storage::Items) -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(config, items);

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<()> {
    let (config_path, items_path) = ensure_data_dir()?;
    let config = Config::load(&config_path)?;
    let provider = JsonProvider { path: items_path };
    let items = provider.load()?;

    if let Err(e) = run(config, items) {
        terminal::disable_raw_mode().ok();
        execute!(io::stdout(), LeaveAlternateScreen).ok();
        return Err(e);
    }

    Ok(())
}
