use std::path::PathBuf;

use anyhow::Result;

use super::{DataProvider, Items};

pub struct JsonProvider {
    pub path: PathBuf,
}

impl DataProvider for JsonProvider {
    fn load(&self) -> Result<Items> {
        let content = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn save(&self, items: &Items) -> Result<()> {
        let content = serde_json::to_string_pretty(items)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }
}
