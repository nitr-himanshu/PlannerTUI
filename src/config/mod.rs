use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    pub columns: u8,
    pub rows: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPosition {
    pub col: u8,
    pub row: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellSpan {
    pub col_span: u8,
    pub row_span: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PanelType {
    Task,
    Jira,
    GithubPr,
    GithubIssue,
    Timer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerWidget {
    pub mode: String,
    pub duration_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: String,
    pub cell: CellPosition,
    pub span: CellSpan,
    #[serde(rename = "type")]
    pub panel_type: PanelType,
    pub widget: Option<TimerWidget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub grid: GridConfig,
    pub panels: Vec<Panel>,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

pub mod defaults;
