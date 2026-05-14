use super::{CellPosition, CellSpan, Config, GridConfig, Panel, PanelType, TimerWidget};
use crate::storage::Items;

pub fn default_config() -> Config {
    Config {
        grid: GridConfig { columns: 2, rows: 2 },
        panels: vec![
            Panel {
                id: "panel-1".to_string(),
                cell: CellPosition { col: 0, row: 0 },
                span: CellSpan { col_span: 1, row_span: 1 },
                panel_type: PanelType::Task,
                widget: None,
            },
            Panel {
                id: "panel-2".to_string(),
                cell: CellPosition { col: 1, row: 0 },
                span: CellSpan { col_span: 1, row_span: 1 },
                panel_type: PanelType::Jira,
                widget: None,
            },
            Panel {
                id: "panel-3".to_string(),
                cell: CellPosition { col: 0, row: 1 },
                span: CellSpan { col_span: 1, row_span: 1 },
                panel_type: PanelType::GithubPr,
                widget: None,
            },
            Panel {
                id: "panel-4".to_string(),
                cell: CellPosition { col: 1, row: 1 },
                span: CellSpan { col_span: 1, row_span: 1 },
                panel_type: PanelType::Timer,
                widget: Some(TimerWidget {
                    mode: "countdown".to_string(),
                    duration_seconds: 1500,
                }),
            },
        ],
    }
}

pub fn default_items() -> Items {
    Items::default()
}
