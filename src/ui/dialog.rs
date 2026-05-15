use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::app::{App, PRIORITY_OPTIONS, TASK_COLORS, TIMER_MODES};

pub fn render_edit(frame: &mut Frame, app: &App) {
    let area = centered_rect(68, 75, frame.area());
    frame.render_widget(Clear, area);

    let title = if app.is_adding { " Add Item " } else { " Edit Item " };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner.inner(Margin::new(1, 1)));

    let items: Vec<ListItem> = app
        .edit_fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let is_focused = i == app.edit_field_cursor;
            match f.label {
                "Priority" => priority_item(&f.value, is_focused),
                "Color" => color_item(&f.value, is_focused),
                "Mode" => mode_item(&f.value, is_focused),
                _ => text_item(f.label, &f.value, is_focused),
            }
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().bg(Color::Rgb(35, 35, 55)));
    let mut state = ListState::default();
    state.select(Some(app.edit_field_cursor));
    frame.render_stateful_widget(list, chunks[0], &mut state);

    let is_cycle = app.is_current_field_cycle();
    let hints = vec![Line::from(vec![
        Span::styled(
            if is_cycle { "← →" } else { "↑↓ / Tab" },
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw(if is_cycle { "  change   " } else { "  next field   " }),
        Span::styled("Ctrl+S", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw("  save   "),
        Span::styled("Esc", Style::default().fg(Color::DarkGray)),
        Span::raw("  cancel"),
    ])];
    frame.render_widget(
        Paragraph::new(Text::from(hints)).alignment(Alignment::Center),
        chunks[1],
    );
}

fn text_item(label: &str, value: &str, is_focused: bool) -> ListItem<'static> {
    let label_span = Span::styled(
        format!("{label:<14}"),
        Style::default().fg(if is_focused { Color::Yellow } else { Color::DarkGray }),
    );
    let value_display = if is_focused {
        format!("{value}_")
    } else {
        value.to_string()
    };
    let value_span = Span::styled(
        value_display,
        Style::default().fg(if is_focused { Color::White } else { Color::Gray }),
    );
    ListItem::new(Line::from(vec![label_span, Span::raw(": "), value_span]))
}

fn priority_item(value: &str, is_focused: bool) -> ListItem<'static> {
    let label_span = Span::styled(
        format!("{:<14}", "Priority"),
        Style::default().fg(if is_focused { Color::Yellow } else { Color::DarkGray }),
    );

    let priority_colors = [Color::Green, Color::Yellow, Color::Red, Color::Magenta];

    let mut spans = vec![label_span, Span::raw(": ")];

    for (opt, &color) in PRIORITY_OPTIONS.iter().zip(priority_colors.iter()) {
        let is_selected = *opt == value;
        if is_selected {
            spans.push(Span::styled(
                format!("[{opt}]"),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                opt.to_string(),
                Style::default().fg(Color::DarkGray),
            ));
        }
        spans.push(Span::raw("  "));
    }

    ListItem::new(Line::from(spans))
}

fn color_item(value: &str, is_focused: bool) -> ListItem<'static> {
    let label_span = Span::styled(
        format!("{:<14}", "Color"),
        Style::default().fg(if is_focused { Color::Yellow } else { Color::DarkGray }),
    );

    let mut spans = vec![label_span, Span::raw(": ")];

    for (hex, name) in TASK_COLORS {
        let color = hex_color(hex);
        let is_selected = *hex == value;
        if is_selected {
            spans.push(Span::styled("[", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
            spans.push(Span::styled(
                format!("██ {name}"),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled("]", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
        } else {
            spans.push(Span::styled("██", Style::default().fg(color)));
        }
        spans.push(Span::raw(" "));
    }

    ListItem::new(Line::from(spans))
}

fn mode_item(value: &str, is_focused: bool) -> ListItem<'static> {
    let label_span = Span::styled(
        format!("{:<14}", "Mode"),
        Style::default().fg(if is_focused { Color::Yellow } else { Color::DarkGray }),
    );
    let mut spans = vec![label_span, Span::raw(": ")];
    for mode in TIMER_MODES {
        let is_selected = *mode == value;
        if is_selected {
            spans.push(Span::styled(
                format!("[{mode}]"),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(mode.to_string(), Style::default().fg(Color::DarkGray)));
        }
        spans.push(Span::raw("  "));
    }
    ListItem::new(Line::from(spans))
}

fn hex_color(hex: &str) -> Color {
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

pub fn render_delete_confirm(frame: &mut Frame, app: &App) {
    let area = centered_rect(52, 30, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(
            " Delete Item ",
            Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(2, 1));
    let title = app.selected_item_title().unwrap_or_else(|| "this item".to_string());

    let lines = vec![
        Line::from(Span::styled("Are you sure you want to delete:", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(
            format!("  \"{title}\""),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" confirm   "),
            Span::styled("Esc", Style::default().fg(Color::DarkGray)),
            Span::raw(" cancel"),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(Text::from(lines)).alignment(Alignment::Center),
        content,
    );
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vert[1])[1]
}
