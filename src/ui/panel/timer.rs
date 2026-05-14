use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use tui_big_text::{BigText, PixelSize};

use crate::widget::timer::TimerState;

fn pick_pixel_size(width: u16, height: u16) -> PixelSize {
    if width >= 40 && height >= 8 {
        PixelSize::Full
    } else if width >= 40 && height >= 4 {
        PixelSize::HalfHeight
    } else if width >= 20 && height >= 8 {
        PixelSize::HalfWidth
    } else {
        PixelSize::Quadrant
    }
}

fn text_rows(size: &PixelSize) -> u16 {
    match size {
        PixelSize::Full | PixelSize::HalfWidth => 8,
        _ => 4,
    }
}

pub fn render(frame: &mut Frame, rect: Rect, timer: Option<&TimerState>, is_active: bool) {
    let (border_style, title_style) = if is_active {
        (
            Style::default().fg(Color::Cyan),
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        )
    } else {
        (Style::default().fg(Color::White), Style::default().fg(Color::White))
    };

    let block = Block::default()
        .title(Span::styled(" Timer ", title_style))
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let content = inner.inner(Margin::new(2, 1));
    if content.width == 0 || content.height == 0 {
        return;
    }

    let (time_str, running, color) = match timer {
        Some(t) => (t.format(), t.running, Color::White),
        None => ("00:00".to_string(), false, Color::DarkGray),
    };

    let big_height = content.height.saturating_sub(1);
    let big_area = Rect::new(content.x, content.y, content.width, big_height);
    let status_area = Rect::new(content.x, content.y + big_height, content.width, 1);

    if big_area.height > 0 {
        let size = pick_pixel_size(big_area.width, big_area.height);
        let rows = text_rows(&size).min(big_area.height);
        let y = big_area.y + big_area.height.saturating_sub(rows) / 2;
        let render_area = Rect::new(big_area.x, y, big_area.width, rows);

        let big_text = BigText::builder()
            .pixel_size(size)
            .alignment(Alignment::Center)
            .lines(vec![Line::from(time_str)])
            .style(Style::default().fg(color))
            .build();

        frame.render_widget(big_text, render_area);
    }

    let indicator = if running { "▶  Running" } else { "⏸  Paused" };
    let status = Paragraph::new(indicator)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(status, status_area);
}
