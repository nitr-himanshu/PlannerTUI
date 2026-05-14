mod config;
mod model;
mod storage;
mod ui;

use anyhow::Result;
use std::path::PathBuf;

use config::Config;
use storage::{json::JsonProvider, DataProvider};

fn data_dir() -> Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("cannot determine executable directory"))?;
    Ok(dir.join(".planner_tui"))
}

fn ensure_data_dir() -> Result<(PathBuf, PathBuf)> {
    let data_dir = data_dir()?;
    std::fs::create_dir_all(&data_dir)?;

    let config_path = data_dir.join("config.json");
    let items_path = data_dir.join("items.json");

    if !config_path.exists() {
        let default = config::defaults::default_config();
        default.save(&config_path)?;
        println!("Created default config: {}", config_path.display());
    }

    if !items_path.exists() {
        let default = config::defaults::default_items();
        let provider = JsonProvider { path: items_path.clone() };
        provider.save(&default)?;
        println!("Created default items: {}", items_path.display());
    }

    Ok((config_path, items_path))
}

fn main() -> Result<()> {
    let (config_path, items_path) = ensure_data_dir()?;

    let config = Config::load(&config_path)?;
    let provider = JsonProvider { path: items_path };
    let items = provider.load()?;

    println!(
        "Loaded {} panel(s) | {} task(s) | {} JIRA item(s) | {} PR(s) | {} issue(s)",
        config.panels.len(),
        items.tasks.len(),
        items.jira.len(),
        items.github_prs.len(),
        items.github_issues.len(),
    );

    Ok(())
}
