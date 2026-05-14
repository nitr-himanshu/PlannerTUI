use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::config::PanelType;

const LINK_LABELS: &[&str] = &["Link", "link"];

fn is_link_label(label: &str) -> bool {
    LINK_LABELS.contains(&label)
}

pub fn render(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 80, frame.area());
    frame.render_widget(Clear, area);

    let type_label = match app.active_panel_type() {
        Some(PanelType::Task) => "Task",
        Some(PanelType::Jira) => "JIRA",
        Some(PanelType::GithubPr) => "GitHub PR",
        Some(PanelType::GithubIssue) => "GitHub Issue",
        _ => "Item",
    };

    let has_link = app.primary_link().is_some();

    let block = Block::default()
        .title(Span::styled(
            format!(" {type_label} "),
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner.inner(Margin::new(2, 1)));

    let fields = app.detail_fields();
    let mut lines: Vec<Line> = Vec::new();

    for (label, value) in &fields {
        if value.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(format!("{label:<14}"), Style::default().fg(Color::DarkGray)),
                Span::styled("—", Style::default().fg(Color::DarkGray)),
            ]));
        } else if is_link_label(label) {
            lines.push(Line::from(vec![
                Span::styled(format!("{label:<14}"), Style::default().fg(Color::Cyan)),
                Span::styled(
                    value.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("{label:<14}"), Style::default().fg(Color::Cyan)),
                Span::styled(value.clone(), Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::from(""));
    }

    frame.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        chunks[0],
    );

    let hint = if has_link {
        vec![
            Span::styled("o", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(": open link   ", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::DarkGray)),
            Span::styled(": back", Style::default().fg(Color::DarkGray)),
        ]
    } else {
        vec![
            Span::styled("Esc", Style::default().fg(Color::DarkGray)),
            Span::styled(": back", Style::default().fg(Color::DarkGray)),
        ]
    };

    frame.render_widget(
        Paragraph::new(Line::from(hint)).alignment(Alignment::Right),
        chunks[1],
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
