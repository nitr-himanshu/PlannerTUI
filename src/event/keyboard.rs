use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;

pub fn handle(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Tab => app.next_panel(),
        KeyCode::BackTab => app.prev_panel(),
        KeyCode::Down => app.scroll_down(),
        KeyCode::Up => app.scroll_up(),
        KeyCode::Char(' ') => app.toggle_active_timer(),
        KeyCode::Char('r') => app.reset_active_timer(),
        _ => {}
    }
    false
}
