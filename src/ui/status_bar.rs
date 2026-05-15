use chrono::Local;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, AppMode};
use crate::config::PanelType;

pub fn render(frame: &mut Frame, rect: Rect, app: &App) {
    let panel_name = app
        .config
        .panels
        .get(app.active_panel)
        .map(|p| p.id.as_str())
        .unwrap_or("none");

    let time = Local::now().format("%H:%M:%S").to_string();

    let hint = match app.mode {
        AppMode::List if matches!(app.active_panel_type(), Some(PanelType::Timer)) => {
            format!(" {panel_name} │ e: edit timer │ Space: start/pause │ r: reset │ Tab: next │ q: quit │ {time}")
        }
        AppMode::List => format!(
            " {panel_name} │ Tab: next │ ↑↓: select │ Enter: view │ a: add │ e: edit │ d: delete │ q: quit │ {time}"
        ),
        AppMode::Detail => {
            if app.primary_link().is_some() {
                format!(" {panel_name} │ o: open link │ Esc: back │ {time}")
            } else {
                format!(" {panel_name} │ Esc: back │ {time}")
            }
        }
        AppMode::Edit => format!(
            " {panel_name} │ ↑↓ / Tab: next field │ Ctrl+S: save │ Esc: cancel │ {time}"
        ),
        AppMode::DeleteConfirm => {
            format!(" {panel_name} │ Enter: confirm delete │ Esc: cancel │ {time}")
        }
    };

    let paragraph =
        Paragraph::new(hint).style(Style::default().fg(Color::Black).bg(Color::White));

    frame.render_widget(paragraph, rect);
}
