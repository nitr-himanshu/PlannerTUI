use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::model::github::GithubIssue;

pub fn render(frame: &mut Frame, rect: Rect, issues: &[GithubIssue], is_active: bool, selected: Option<usize>) {
    let (border_style, title_style) = if is_active {
        (
            Style::default().fg(Color::Cyan),
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        )
    } else {
        (Style::default().fg(Color::White), Style::default().fg(Color::White))
    };

    let block = Block::default()
        .title(Span::styled(" GitHub Issues ", title_style))
        .borders(Borders::ALL)
        .border_style(border_style);

    let items: Vec<ListItem> = issues
        .iter()
        .map(|issue| {
            let line = Line::from(vec![
                Span::styled(format!("{} ", issue.id), Style::default().fg(Color::Yellow)),
                Span::styled(issue.link.clone(), Style::default().fg(Color::DarkGray)),
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
