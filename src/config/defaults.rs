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
    use crate::model::{
        github::{GithubIssue, GithubPr},
        jira::JiraItem,
        task::{Priority, Task},
    };
    Items {
        tasks: vec![
            Task {
                id: "task-1".to_string(),
                title: "Set up project structure".to_string(),
                description: String::new(),
                deadline: "2026-05-20T18:00:00".to_string(),
                priority: Priority::High,
                color: "#FF6B6B".to_string(),
            },
            Task {
                id: "task-2".to_string(),
                title: "Implement grid layout".to_string(),
                description: String::new(),
                deadline: "2026-05-25T18:00:00".to_string(),
                priority: Priority::Medium,
                color: "#4ECDC4".to_string(),
            },
        ],
        jira: vec![JiraItem {
            id: "PROJ-1".to_string(),
            title: "API rate limiting".to_string(),
            link: "https://your-org.atlassian.net/browse/PROJ-1".to_string(),
            description: "Implement rate limiting on public endpoints".to_string(),
            comment: "Token bucket algorithm proposed".to_string(),
        }],
        github_prs: vec![GithubPr {
            id: "org/repo#42".to_string(),
            link: "https://github.com/org/repo/pull/42".to_string(),
            description: String::new(),
        }],
        github_issues: vec![GithubIssue {
            id: "org/repo#13".to_string(),
            link: "https://github.com/org/repo/issues/13".to_string(),
            description: String::new(),
        }],
    }
}
