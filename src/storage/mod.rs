use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::model::{
    github::{GithubIssue, GithubPr},
    jira::JiraItem,
    task::Task,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Items {
    pub tasks: Vec<Task>,
    pub jira: Vec<JiraItem>,
    pub github_prs: Vec<GithubPr>,
    pub github_issues: Vec<GithubIssue>,
}

pub trait DataProvider {
    fn load(&self) -> Result<Items>;
    fn save(&self, items: &Items) -> Result<()>;
}

pub mod json;
