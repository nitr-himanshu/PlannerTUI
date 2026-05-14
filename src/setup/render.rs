use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use super::state::{SetupState, SetupStep};

pub fn render(frame: &mut Frame, state: &SetupState) {
    let area = frame.area();

    let bg = Paragraph::new("").style(Style::default().bg(Color::Rgb(15, 15, 25)));
    frame.render_widget(bg, area);

    let card = centered_rect(72, 88, area);
    frame.render_widget(Clear, card);

    match state.step {
        SetupStep::Welcome => render_welcome(frame, card),
        SetupStep::FeatureSelect => render_feature_select(frame, card, state),
        SetupStep::LayoutPreview => render_layout_preview(frame, card, state),
        SetupStep::ConfigGuide => render_config_guide(frame, card),
        SetupStep::ItemsGuide => render_items_guide(frame, card),
        SetupStep::SkillGuide => render_skill_guide(frame, card),
        SetupStep::Complete => render_complete(frame, card),
    }
}

fn render_welcome(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(3, 2));

    let lines = vec![
        Line::from(Span::styled(
            "PlannerTUI",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your customizable terminal dashboard for developers.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Track your work from multiple sources — all in one place:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✦  ", Style::default().fg(Color::Cyan)),
            Span::raw("GitHub PRs & Issues"),
        ]),
        Line::from(vec![
            Span::styled("  ✦  ", Style::default().fg(Color::Cyan)),
            Span::raw("JIRA tickets"),
        ]),
        Line::from(vec![
            Span::styled("  ✦  ", Style::default().fg(Color::Cyan)),
            Span::raw("Focus timer"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Grid layout is fully configurable — resize, merge cells, rearrange any time.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Let's set up your dashboard in a few steps.",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
    ];

    let text = Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim: false })
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    frame.render_widget(text, content);

    render_hint(frame, inner, "Press any key to begin  →");
}

fn render_feature_select(frame: &mut Frame, area: Rect, state: &SetupState) {
    let block = Block::default()
        .title(Span::styled(
            " Which panels would you like? ",
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(2, 1));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(content);

    let subtitle = Paragraph::new(Span::styled(
        "Select at least one panel  (Space to toggle, ↑↓ to move, Enter to confirm)",
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(subtitle, chunks[0]);

    let items: Vec<ListItem> = state
        .features
        .iter()
        .map(|f| {
            let checkbox = if f.selected {
                Span::styled("[✓] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
            };
            let label = Span::styled(
                f.label,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            );
            let desc = Span::styled(
                format!("  —  {}", f.description),
                Style::default().fg(Color::DarkGray),
            );
            ListItem::new(Line::from(vec![checkbox, label, desc]))
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().bg(Color::Rgb(40, 40, 60)).fg(Color::Cyan));
    let mut list_state = ListState::default();
    list_state.select(Some(state.cursor));
    frame.render_stateful_widget(list, chunks[2], &mut list_state);

    if state.no_selection_hint {
        let hint = Paragraph::new(Span::styled(
            "  ⚠  Select at least one panel to continue.",
            Style::default().fg(Color::Yellow),
        ));
        frame.render_widget(hint, chunks[3]);
    }
}

fn render_layout_preview(frame: &mut Frame, area: Rect, state: &SetupState) {
    let n = state.selected_count();
    let opt = state.layout_option;

    let (grid_label, art) = layout_art(state, n, opt);

    let block = Block::default()
        .title(Span::styled(
            " Suggested Layout ",
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(3, 1));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(content);

    let header_lines = vec![
        Line::from(vec![
            Span::styled("Layout:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(grid_label, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];
    frame.render_widget(Paragraph::new(Text::from(header_lines)), chunks[0]);

    let art_lines: Vec<Line> = art.iter().map(|l| Line::from(Span::styled(
        l.as_str(),
        Style::default().fg(Color::White),
    ))).collect();
    frame.render_widget(Paragraph::new(Text::from(art_lines)), chunks[1]);

    let nav = if state.layout_options_count() > 1 {
        let left = if opt > 0 { "← " } else { "  " };
        let right = if opt + 1 < state.layout_options_count() { " →" } else { "  " };
        format!("{}Option {}/{}{}", left, opt + 1, state.layout_options_count(), right)
    } else {
        String::new()
    };

    let footer_lines = vec![
        Line::from(Span::styled(nav, Style::default().fg(Color::DarkGray))),
    ];
    frame.render_widget(Paragraph::new(Text::from(footer_lines)), chunks[2]);

    render_hint(frame, inner, "Enter to confirm");
}

fn render_config_guide(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " Customising Your Layout ",
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(3, 2));

    let lines = vec![
        Line::from(vec![
            Span::styled("Your grid config lives at  ", Style::default().fg(Color::Gray)),
            Span::styled(".planner_tui/config.json", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled("What you can change:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  grid.columns / grid.rows  ", Style::default().fg(Color::Cyan)),
            Span::styled("— set the grid dimensions (max 4 × 2)", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  panels[].type             ", Style::default().fg(Color::Cyan)),
            Span::styled("— task / jira / github_pr / github_issue / timer", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  panels[].cell             ", Style::default().fg(Color::Cyan)),
            Span::styled("— { col, row } zero-indexed position", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  panels[].span             ", Style::default().fg(Color::Cyan)),
            Span::styled("— { col_span, row_span } to merge cells", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Edit the file and restart to apply changes.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        content,
    );

    render_hint(frame, inner, "Press any key to continue  →");
}

fn render_items_guide(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " Adding Your Data ",
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(3, 2));

    let lines = vec![
        Line::from(vec![
            Span::styled("Your data lives at  ", Style::default().fg(Color::Gray)),
            Span::styled(".planner_tui/items.json", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled("Four sections — add entries to any:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  tasks         ", Style::default().fg(Color::Cyan)),
            Span::styled("— id, title, deadline, priority (Low/Medium/High/Critical), color (#hex)", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  jira          ", Style::default().fg(Color::Cyan)),
            Span::styled("— id, title, link, description, comment", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  github_prs    ", Style::default().fg(Color::Cyan)),
            Span::styled("— id, link", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("  github_issues ", Style::default().fg(Color::Cyan)),
            Span::styled("— id, link", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Edit this file manually, or use the AI sync skill to auto-populate it.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        content,
    );

    render_hint(frame, inner, "Press any key to continue  →");
}

fn render_skill_guide(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(Span::styled(
            " AI-Powered Sync ",
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(3, 2));

    let lines = vec![
        Line::from(Span::styled(
            "Auto-populate items.json with live data from GitHub & JIRA:",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  1.  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Open "),
            Span::styled("docs/sync-skill.md", Style::default().fg(Color::Cyan)),
            Span::raw(" in Claude"),
        ]),
        Line::from(vec![
            Span::styled("  2.  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Say: "),
            Span::styled("\"Follow the sync skill\"", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("  3.  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Claude detects your MCP connections (GitHub, JIRA)"),
        ]),
        Line::from(vec![
            Span::styled("       ", Style::default().fg(Color::Cyan)),
            Span::styled("No credentials needed — uses your existing MCP session", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  4.  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Claude asks for filters (repos, JQL query, etc.)"),
        ]),
        Line::from(vec![
            Span::styled("  5.  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Fetches data and writes "),
            Span::styled("items.json", Style::default().fg(Color::Cyan)),
            Span::raw(" automatically"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Re-run any time to refresh your dashboard.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        content,
    );

    render_hint(frame, inner, "Press any key to finish  →");
}

fn render_complete(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .style(Style::default().bg(Color::Rgb(20, 20, 35)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = inner.inner(Margin::new(3, 2));

    let lines = vec![
        Line::from(Span::styled(
            "✓  All set!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your dashboard is configured and ready to launch.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  config.json ", Style::default().fg(Color::Cyan)),
            Span::styled("created  ✓", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  items.json  ", Style::default().fg(Color::Cyan)),
            Span::styled("created  ✓", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Tip: populate items.json using docs/sync-skill.md with Claude.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(
        Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false }),
        content,
    );

    render_hint(frame, inner, "Press Enter to launch PlannerTUI  →");
}

fn render_hint(frame: &mut Frame, area: Rect, text: &str) {
    let hint_area = Rect::new(area.x, area.y + area.height.saturating_sub(2), area.width, 1);
    let hint = Paragraph::new(Span::styled(text, Style::default().fg(Color::DarkGray)))
        .alignment(Alignment::Right);
    frame.render_widget(hint, hint_area);
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

fn c1(s: &str) -> String {
    let s = if s.len() > 6 { &s[..6] } else { s };
    format!("{:^6}", s)
}

fn c2(s: &str) -> String {
    let s = if s.len() > 17 { &s[..17] } else { s };
    format!("{:^17}", s)
}

fn layout_art(state: &SetupState, count: usize, option: usize) -> (String, Vec<String>) {
    let names: Vec<&str> = state
        .features
        .iter()
        .filter(|f| f.selected)
        .map(|f| f.short)
        .collect();

    let g = |i: usize| names.get(i).copied().unwrap_or("?");

    match (count, option) {
        (1, _) => (
            "1 column × 1 row".to_string(),
            vec![
                "┌──────────┐".into(),
                "│          │".into(),
                format!("│  {}  │", c1(g(0))),
                "│          │".into(),
                "└──────────┘".into(),
            ],
        ),
        (2, _) => (
            "2 columns × 1 row".to_string(),
            vec![
                "┌──────────┬──────────┐".into(),
                "│          │          │".into(),
                format!("│  {}  │  {}  │", c1(g(0)), c1(g(1))),
                "│          │          │".into(),
                "└──────────┴──────────┘".into(),
            ],
        ),
        (3, 0) => (
            "3 columns × 1 row".to_string(),
            vec![
                "┌──────────┬──────────┬──────────┐".into(),
                "│          │          │          │".into(),
                format!("│  {}  │  {}  │  {}  │", c1(g(0)), c1(g(1)), c1(g(2))),
                "│          │          │          │".into(),
                "└──────────┴──────────┴──────────┘".into(),
            ],
        ),
        (3, _) => (
            "2 columns — left panel spans both rows".to_string(),
            vec![
                "┌──────────┬──────────┐".into(),
                "│          │          │".into(),
                format!("│          │  {}  │", c1(g(1))),
                format!("│  {}  ├──────────┤", c1(g(0))),
                format!("│          │  {}  │", c1(g(2))),
                "│          │          │".into(),
                "└──────────┴──────────┘".into(),
            ],
        ),
        (4, 0) => (
            "2 columns × 2 rows".to_string(),
            vec![
                "┌──────────┬──────────┐".into(),
                "│          │          │".into(),
                format!("│  {}  │  {}  │", c1(g(0)), c1(g(1))),
                "├──────────┼──────────┤".into(),
                format!("│  {}  │  {}  │", c1(g(2)), c1(g(3))),
                "│          │          │".into(),
                "└──────────┴──────────┘".into(),
            ],
        ),
        (4, _) => (
            "3 columns × 2 rows — wide top-left".to_string(),
            vec![
                "┌─────────────────────┬──────────┐".into(),
                "│                     │          │".into(),
                format!("│  {}  │  {}  │", c2(g(0)), c1(g(1))),
                "├──────────┬──────────┴──────────┤".into(),
                format!("│  {}  │  {}  │", c1(g(2)), c2(g(3))),
                "│          │                     │".into(),
                "└──────────┴─────────────────────┘".into(),
            ],
        ),
        (5, 0) => (
            "3 columns × 2 rows".to_string(),
            vec![
                "┌──────────┬──────────┬──────────┐".into(),
                "│          │          │          │".into(),
                format!("│  {}  │  {}  │  {}  │", c1(g(0)), c1(g(1)), c1(g(2))),
                "├──────────┼──────────┴──────────┤".into(),
                format!("│  {}  │  {}  │", c1(g(3)), c2(g(4))),
                "│          │                     │".into(),
                "└──────────┴─────────────────────┘".into(),
            ],
        ),
        (5, _) => (
            "3 columns × 2 rows — wide top-left".to_string(),
            vec![
                "┌─────────────────────┬──────────┐".into(),
                "│                     │          │".into(),
                format!("│  {}  │  {}  │", c2(g(0)), c1(g(1))),
                "├──────────┬──────────┼──────────┤".into(),
                format!("│  {}  │  {}  │  {}  │", c1(g(2)), c1(g(3)), c1(g(4))),
                "│          │          │          │".into(),
                "└──────────┴──────────┴──────────┘".into(),
            ],
        ),
        _ => ("unknown".to_string(), vec![]),
    }
}
