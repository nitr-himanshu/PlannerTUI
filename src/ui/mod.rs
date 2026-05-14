pub mod detail;
pub mod dialog;
pub mod grid;
pub mod panel;
pub mod status_bar;

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, AppMode};
use crate::config::PanelType;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let main_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(1));
    let status_area = Rect::new(area.x, area.y + area.height.saturating_sub(1), area.width, 1);

    match grid::resolve(&app.config, main_area) {
        Ok(resolved_panels) => {
            for (i, resolved) in resolved_panels.iter().enumerate() {
                let is_active = i == app.active_panel;
                let selected = app.get_panel_selected(&resolved.id);
                match resolved.panel_type {
                    PanelType::Task => {
                        panel::task::render(frame, resolved.rect, &app.items.tasks, is_active, selected)
                    }
                    PanelType::Jira => {
                        panel::jira::render(frame, resolved.rect, &app.items.jira, is_active, selected)
                    }
                    PanelType::GithubPr => {
                        panel::github_pr::render(frame, resolved.rect, &app.items.github_prs, is_active, selected)
                    }
                    PanelType::GithubIssue => {
                        panel::github_issue::render(frame, resolved.rect, &app.items.github_issues, is_active, selected)
                    }
                    PanelType::Timer => {
                        let timer = app.timers.get(&resolved.id);
                        panel::timer::render(frame, resolved.rect, timer, is_active)
                    }
                }
            }
        }
        Err(e) => {
            let msg = Paragraph::new(format!("Config error: {e}"))
                .style(Style::default().fg(Color::Red));
            frame.render_widget(msg, main_area);
        }
    }

    status_bar::render(frame, status_area, app);

    match app.mode {
        AppMode::List => {}
        AppMode::Detail => detail::render(frame, app),
        AppMode::Edit => dialog::render_edit(frame, app),
        AppMode::DeleteConfirm => dialog::render_delete_confirm(frame, app),
    }
}
