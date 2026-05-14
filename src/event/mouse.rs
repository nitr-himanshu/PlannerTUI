use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::app::{App, AppMode};

pub fn handle(event: MouseEvent, app: &mut App, panel_layout: &[(usize, Rect)]) {
    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if matches!(app.mode, AppMode::List) {
                for &(i, rect) in panel_layout {
                    if event.column >= rect.x
                        && event.column < rect.x + rect.width
                        && event.row >= rect.y
                        && event.row < rect.y + rect.height
                    {
                        app.active_panel = i;
                        break;
                    }
                }
            }
        }
        MouseEventKind::ScrollDown if matches!(app.mode, AppMode::List) => app.select_down(),
        MouseEventKind::ScrollUp if matches!(app.mode, AppMode::List) => app.select_up(),
        _ => {}
    }
}
