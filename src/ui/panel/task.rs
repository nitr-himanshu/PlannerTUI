use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::model::task::{Priority, Task};

fn priority_color(priority: &Priority) -> Color {
    match priority {
        Priority::Low => Color::Green,
        Priority::Medium => Color::Yellow,
        Priority::High => Color::Red,
        Priority::Critical => Color::Magenta,
    }
}

fn priority_label(priority: &Priority) -> &'static str {
    match priority {
        Priority::Low => "LOW ",
        Priority::Medium => "MED ",
        Priority::High => "HIGH",
        Priority::Critical => "CRIT",
    }
}

fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6
        && let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        )
    {
        return Color::Rgb(r, g, b);
    }
    Color::White
}

pub fn render(frame: &mut Frame, rect: Rect, tasks: &[Task], is_active: bool, selected: Option<usize>) {
    let (border_style, title_style) = if is_active {
        (
            Style::default().fg(Color::Cyan),
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        )
    } else {
        (Style::default().fg(Color::White), Style::default().fg(Color::White))
    };

    let block = Block::default()
        .title(Span::styled(" Tasks ", title_style))
        .borders(Borders::ALL)
        .border_style(border_style);

    let items: Vec<ListItem> = tasks
        .iter()
        .map(|task| {
            let color = parse_hex_color(&task.color);
            let prio_color = priority_color(&task.priority);
            let label = priority_label(&task.priority);
            let line = Line::from(vec![
                Span::styled("█ ", Style::default().fg(color)),
                Span::styled(format!("[{label}] "), Style::default().fg(prio_color)),
                Span::raw(task.title.clone()),
                Span::styled(
                    format!("  {}", task.deadline),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default().bg(Color::Rgb(40, 60, 80)).add_modifier(Modifier::BOLD),
        );
    let mut state = ListState::default();
    state.select(selected);
    frame.render_stateful_widget(list, rect, &mut state);
}
