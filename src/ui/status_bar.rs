use chrono::Local;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, rect: Rect, app: &App) {
    let panel_name = app
        .config
        .panels
        .get(app.active_panel)
        .map(|p| p.id.as_str())
        .unwrap_or("none");

    let time = Local::now().format("%H:%M:%S").to_string();
    let text = format!(" {panel_name} │ Tab: next │ ↑↓: scroll │ q: quit │ {time}");

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Black).bg(Color::White));

    frame.render_widget(paragraph, rect);
}
