pub mod grid;
pub mod panel;
pub mod status_bar;

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::config::PanelType;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let main_area = Rect::new(area.x, area.y, area.width, area.height.saturating_sub(1));
    let status_area = Rect::new(area.x, area.y + area.height.saturating_sub(1), area.width, 1);

    match grid::resolve(&app.config, main_area) {
        Ok(resolved_panels) => {
            for (i, resolved) in resolved_panels.iter().enumerate() {
                let is_active = i == app.active_panel;
                let scroll = app.scroll_offsets.get(&resolved.id).copied().unwrap_or(0);
                match resolved.panel_type {
                    PanelType::Task => {
                        panel::task::render(frame, resolved.rect, &app.items.tasks, is_active, scroll)
                    }
                    PanelType::Jira => {
                        panel::jira::render(frame, resolved.rect, &app.items.jira, is_active, scroll)
                    }
                    PanelType::GithubPr => {
                        panel::github_pr::render(frame, resolved.rect, &app.items.github_prs, is_active, scroll)
                    }
                    PanelType::GithubIssue => {
                        panel::github_issue::render(frame, resolved.rect, &app.items.github_issues, is_active, scroll)
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
}
