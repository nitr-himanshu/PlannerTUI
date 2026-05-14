mod config_gen;
mod render;
mod state;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use crate::config::{self, Config};
use crate::storage::Items;
use state::{SetupState, SetupStep};

pub struct SetupResult {
    pub config: Config,
    pub items: Items,
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<SetupResult> {
    let mut state = SetupState::new();

    loop {
        terminal.draw(|frame| render::render(frame, &state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return Err(anyhow::anyhow!("Setup cancelled"));
            }

            match state.step {
                SetupStep::FeatureSelect => match key.code {
                    KeyCode::Up => state.prev_feature(),
                    KeyCode::Down => state.next_feature(),
                    KeyCode::Char(' ') => state.toggle_feature(),
                    KeyCode::Enter => {
                        state.advance();
                    }
                    _ => {}
                },
                SetupStep::LayoutPreview => match key.code {
                    KeyCode::Left => state.prev_layout(),
                    KeyCode::Right => state.next_layout(),
                    KeyCode::Enter => {
                        state.advance();
                    }
                    _ => {}
                },
                SetupStep::Complete => {
                    if key.code == KeyCode::Enter {
                        break;
                    }
                }
                _ => {
                    state.advance();
                }
            }
        }
    }

    Ok(SetupResult {
        config: config_gen::generate(&state),
        items: config::defaults::default_items(),
    })
}
