use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::model::jira::JiraItem;

pub fn render(frame: &mut Frame, rect: Rect, items: &[JiraItem], is_active: bool, scroll: usize) {
    let (border_style, title_style) = if is_active {
        (
            Style::default().fg(Color::Cyan),
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        )
    } else {
        (Style::default().fg(Color::White), Style::default().fg(Color::White))
    };

    let block = Block::default()
        .title(Span::styled(" JIRA ", title_style))
        .borders(Borders::ALL)
        .border_style(border_style);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| {
            let title_line = Line::from(vec![
                Span::styled(format!("[{}] ", item.id), Style::default().fg(Color::Blue)),
                Span::raw(item.title.clone()),
            ]);
            let desc_line = Line::from(Span::styled(
                format!("  {}", item.description),
                Style::default().fg(Color::DarkGray),
            ));
            ListItem::new(Text::from(vec![title_line, desc_line]))
        })
        .collect();

    let list = List::new(list_items).block(block);
    let mut state = ListState::default();
    *state.offset_mut() = scroll.min(items.len().saturating_sub(1));
    frame.render_stateful_widget(list, rect, &mut state);
}
