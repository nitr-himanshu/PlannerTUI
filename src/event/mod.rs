pub mod keyboard;
pub mod mouse;

pub enum AppEvent {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
}
