use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::model::github::GithubPr;

pub fn render(frame: &mut Frame, rect: Rect, prs: &[GithubPr], is_active: bool, scroll: usize) {
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let block = Block::default()
        .title("GitHub PRs")
        .borders(Borders::ALL)
        .border_style(border_style);

    let items: Vec<ListItem> = prs
        .iter()
        .map(|pr| {
            let line = Line::from(vec![
                Span::styled(format!("{} ", pr.id), Style::default().fg(Color::Green)),
                Span::styled(pr.link.clone(), Style::default().fg(Color::DarkGray)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(block);
    let mut state = ListState::default();
    *state.offset_mut() = scroll.min(prs.len().saturating_sub(1));
    frame.render_stateful_widget(list, rect, &mut state);
}
