use std::collections::HashMap;

use crate::config::{Config, PanelType};
use crate::storage::Items;
use crate::widget::timer::TimerState;

pub struct App {
    pub config: Config,
    pub items: Items,
    pub active_panel: usize,
    pub timers: HashMap<String, TimerState>,
}

impl App {
    pub fn new(config: Config, items: Items) -> Self {
        let timers = config
            .panels
            .iter()
            .filter(|p| p.panel_type == PanelType::Timer)
            .map(|p| {
                let state = match &p.widget {
                    Some(w) if w.mode == "countdown" => TimerState::new_countdown(w.duration_seconds),
                    Some(_) => TimerState::new_countup(),
                    None => TimerState::new_countdown(0),
                };
                (p.id.clone(), state)
            })
            .collect();

        Self {
            config,
            items,
            active_panel: 0,
            timers,
        }
    }

    pub fn on_tick(&mut self) {
        for state in self.timers.values_mut() {
            state.tick();
        }
    }

    pub fn toggle_active_timer(&mut self) {
        let id = self.config.panels.get(self.active_panel).map(|p| p.id.clone());
        if let Some(id) = id {
            if let Some(timer) = self.timers.get_mut(&id) {
                timer.toggle();
            }
        }
    }

    pub fn reset_active_timer(&mut self) {
        let id = self.config.panels.get(self.active_panel).map(|p| p.id.clone());
        if let Some(id) = id {
            if let Some(timer) = self.timers.get_mut(&id) {
                timer.reset();
            }
        }
    }
}
