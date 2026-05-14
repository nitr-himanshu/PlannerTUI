use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::app::App;

pub fn handle(event: MouseEvent, app: &mut App, panel_layout: &[(usize, Rect)]) {
    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
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
        MouseEventKind::ScrollDown => app.scroll_down(),
        MouseEventKind::ScrollUp => app.scroll_up(),
        _ => {}
    }
}
