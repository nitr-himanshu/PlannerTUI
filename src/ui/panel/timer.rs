use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render(frame: &mut Frame, rect: Rect, is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let block = Block::default()
        .title("Timer")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let center_y = inner.y + inner.height / 2;
    let text_rect = Rect::new(inner.x, center_y, inner.width, 1);

    let text = Paragraph::new("00:00")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    frame.render_widget(text, text_rect);
}
