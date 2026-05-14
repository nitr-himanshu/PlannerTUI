use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraItem {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub comment: String,
}
