use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::widget::timer::TimerState;

pub fn render(frame: &mut Frame, rect: Rect, timer: Option<&TimerState>, is_active: bool) {
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

    let (display, color) = match timer {
        Some(t) => {
            let indicator = if t.running { "▶  " } else { "⏸  " };
            (format!("{}{}", indicator, t.format()), Color::White)
        }
        None => ("00:00".to_string(), Color::DarkGray),
    };

    if inner.height > 0 {
        let center_y = inner.y + inner.height / 2;
        let text_rect = Rect::new(inner.x, center_y, inner.width, 1);
        let text = Paragraph::new(display)
            .alignment(Alignment::Center)
            .style(Style::default().fg(color));
        frame.render_widget(text, text_rect);
    }
}
