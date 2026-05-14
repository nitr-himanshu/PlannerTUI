use crate::config::Config;
use crate::storage::Items;

pub struct App {
    pub config: Config,
    pub items: Items,
    pub active_panel: usize,
}

impl App {
    pub fn new(config: Config, items: Items) -> Self {
        Self {
            config,
            items,
            active_panel: 0,
        }
    }
}
