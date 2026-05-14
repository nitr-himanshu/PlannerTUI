use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;

use crate::model::jira::JiraItem;

pub fn render(frame: &mut Frame, rect: Rect, items: &[JiraItem], is_active: bool) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let block = Block::default()
        .title("JIRA")
        .borders(Borders::ALL)
        .border_style(border_style);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| {
            let title_line = Line::from(vec![
                Span::styled(
                    format!("[{}] ", item.id),
                    Style::default().fg(Color::Blue),
                ),
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
    frame.render_widget(list, rect);
}
