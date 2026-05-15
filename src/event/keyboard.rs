use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{App, AppMode};
use crate::config::PanelType;

pub fn handle(key: KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    match app.mode {
        AppMode::List => handle_list(key, app),
        AppMode::Detail => handle_detail(key, app),
        AppMode::Edit => handle_edit(key, app),
        AppMode::DeleteConfirm => handle_delete_confirm(key, app),
    }
}

fn handle_list(key: KeyEvent, app: &mut App) -> bool {
    let is_timer = matches!(app.active_panel_type(), Some(PanelType::Timer));
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Tab => app.next_panel(),
        KeyCode::BackTab => app.prev_panel(),
        KeyCode::Down => app.select_down(),
        KeyCode::Up => app.select_up(),
        KeyCode::Enter if !is_timer => app.open_detail(),
        KeyCode::Char('a') if !is_timer => app.open_add(),
        KeyCode::Char('e') => app.open_edit(),
        KeyCode::Char('d') if !is_timer => app.open_delete_confirm(),
        KeyCode::Char(' ') => app.toggle_active_timer(),
        KeyCode::Char('r') => app.reset_active_timer(),
        _ => {}
    }
    false
}

fn handle_detail(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Esc => app.close_detail(),
        KeyCode::Char('o') => app.open_link(),
        _ => {}
    }
    false
}

fn handle_edit(key: KeyEvent, app: &mut App) -> bool {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if key.code == KeyCode::Char('s') {
            app.save_edit();
        }
        return false;
    }
    match key.code {
        KeyCode::Esc => app.mode = AppMode::List,
        KeyCode::Tab | KeyCode::Down => app.next_edit_field(),
        KeyCode::BackTab | KeyCode::Up => app.prev_edit_field(),
        KeyCode::Left => app.cycle_field_prev(),
        KeyCode::Right => app.cycle_field_next(),
        KeyCode::Backspace if !app.is_current_field_cycle() => app.edit_backspace(),
        KeyCode::Char(c) if !app.is_current_field_cycle() => app.type_char(c),
        _ => {}
    }
    false
}

fn handle_delete_confirm(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Enter => app.confirm_delete(),
        KeyCode::Esc => app.mode = AppMode::List,
        _ => {}
    }
    false
}
