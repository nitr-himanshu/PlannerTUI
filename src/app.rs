use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{Config, PanelType};
use crate::model::github::{GithubIssue, GithubPr};
use crate::model::jira::JiraItem;
use crate::model::task::{Priority, Task};
use crate::storage::{json::JsonProvider, DataProvider, Items};
use crate::widget::timer::{TimerMode, TimerState};

pub const PRIORITY_OPTIONS: &[&str] = &["Low", "Medium", "High", "Critical"];
pub const TIMER_MODES: &[&str] = &["Countdown", "Countup"];

pub const TASK_COLORS: &[(&str, &str)] = &[
    ("#FF6B6B", "Coral"),
    ("#FF9F43", "Amber"),
    ("#FECA57", "Yellow"),
    ("#48DBFB", "Sky"),
    ("#1DD1A1", "Mint"),
    ("#54A0FF", "Blue"),
    ("#A29BFE", "Lavender"),
    ("#FD79A8", "Pink"),
    ("#00CEC9", "Teal"),
    ("#FDCB6E", "Gold"),
];

pub enum AppMode {
    List,
    Detail,
    Edit,
    DeleteConfirm,
}

pub struct EditField {
    pub label: &'static str,
    pub value: String,
}

pub struct App {
    pub config: Config,
    pub items: Items,
    pub items_path: PathBuf,
    pub config_path: PathBuf,
    pub active_panel: usize,
    pub timers: HashMap<String, TimerState>,
    pub selected_items: HashMap<String, usize>,
    pub mode: AppMode,
    pub edit_fields: Vec<EditField>,
    pub edit_field_cursor: usize,
    pub is_adding: bool,
}

impl App {
    pub fn new(config: Config, items: Items, items_path: PathBuf, config_path: PathBuf) -> Self {
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
            items_path,
            config_path,
            active_panel: 0,
            timers,
            selected_items: HashMap::new(),
            mode: AppMode::List,
            edit_fields: Vec::new(),
            edit_field_cursor: 0,
            is_adding: false,
        }
    }

    pub fn active_panel_id(&self) -> Option<&str> {
        self.config.panels.get(self.active_panel).map(|p| p.id.as_str())
    }

    pub fn active_panel_type(&self) -> Option<PanelType> {
        self.config.panels.get(self.active_panel).map(|p| p.panel_type)
    }

    fn panel_item_count(&self) -> usize {
        self.items_count_for_type(self.active_panel_type())
    }

    fn items_count_for_type(&self, pt: Option<PanelType>) -> usize {
        match pt {
            Some(PanelType::Task) => self.items.tasks.len(),
            Some(PanelType::Jira) => self.items.jira.len(),
            Some(PanelType::GithubPr) => self.items.github_prs.len(),
            Some(PanelType::GithubIssue) => self.items.github_issues.len(),
            _ => 0,
        }
    }

    fn items_count_for_panel_id(&self, panel_id: &str) -> usize {
        let pt = self.config.panels.iter().find(|p| p.id == panel_id).map(|p| p.panel_type);
        self.items_count_for_type(pt)
    }

    pub fn get_panel_selected(&self, panel_id: &str) -> Option<usize> {
        let count = self.items_count_for_panel_id(panel_id);
        if count == 0 { None } else { Some(self.selected_items.get(panel_id).copied().unwrap_or(0)) }
    }

    pub fn selected_index(&self) -> Option<usize> {
        let count = self.panel_item_count();
        if count == 0 { return None; }
        let id = self.active_panel_id()?;
        Some(self.selected_items.get(id).copied().unwrap_or(0))
    }

    pub fn on_tick(&mut self) {
        for state in self.timers.values_mut() {
            state.tick();
        }
    }

    pub fn next_panel(&mut self) {
        if !self.config.panels.is_empty() {
            self.active_panel = (self.active_panel + 1) % self.config.panels.len();
        }
    }

    pub fn prev_panel(&mut self) {
        if !self.config.panels.is_empty() {
            self.active_panel = if self.active_panel == 0 {
                self.config.panels.len() - 1
            } else {
                self.active_panel - 1
            };
        }
    }

    pub fn select_down(&mut self) {
        let count = self.panel_item_count();
        if count == 0 { return; }
        let id = match self.active_panel_id() { Some(id) => id.to_string(), None => return };
        let cur = self.selected_items.get(&id).copied().unwrap_or(0);
        self.selected_items.insert(id, (cur + 1).min(count - 1));
    }

    pub fn select_up(&mut self) {
        let id = match self.active_panel_id() { Some(id) => id.to_string(), None => return };
        let cur = self.selected_items.get(&id).copied().unwrap_or(0);
        self.selected_items.insert(id, cur.saturating_sub(1));
    }

    pub fn toggle_active_timer(&mut self) {
        let id = self.config.panels.get(self.active_panel).map(|p| p.id.clone());
        if let Some(id) = id
            && let Some(timer) = self.timers.get_mut(&id)
        {
            timer.toggle();
        }
    }

    pub fn reset_active_timer(&mut self) {
        let id = self.config.panels.get(self.active_panel).map(|p| p.id.clone());
        if let Some(id) = id
            && let Some(timer) = self.timers.get_mut(&id)
        {
            timer.reset();
        }
    }

    pub fn open_detail(&mut self) {
        if self.selected_index().is_some() {
            self.mode = AppMode::Detail;
        }
    }

    pub fn close_detail(&mut self) {
        self.mode = AppMode::List;
    }

    pub fn open_add(&mut self) {
        self.is_adding = true;
        self.edit_fields = self.empty_fields_for_active();
        self.edit_field_cursor = 0;
        self.mode = AppMode::Edit;
    }

    pub fn open_edit(&mut self) {
        self.is_adding = false;
        self.edit_fields = self.fields_for_selected();
        self.edit_field_cursor = 0;
        self.mode = AppMode::Edit;
    }

    pub fn open_delete_confirm(&mut self) {
        if self.selected_index().is_some() {
            self.mode = AppMode::DeleteConfirm;
        }
    }

    pub fn next_edit_field(&mut self) {
        if self.edit_field_cursor + 1 < self.edit_fields.len() {
            self.edit_field_cursor += 1;
        }
    }

    pub fn prev_edit_field(&mut self) {
        self.edit_field_cursor = self.edit_field_cursor.saturating_sub(1);
    }

    pub fn is_current_field_cycle(&self) -> bool {
        matches!(
            self.edit_fields.get(self.edit_field_cursor).map(|f| f.label),
            Some("Priority") | Some("Color") | Some("Mode")
        )
    }

    pub fn cycle_field_next(&mut self) {
        match self.edit_fields.get(self.edit_field_cursor).map(|f| f.label) {
            Some("Priority") => self.cycle_priority(1),
            Some("Color") => self.cycle_color(1),
            Some("Mode") => self.cycle_mode(1),
            _ => {}
        }
    }

    pub fn cycle_field_prev(&mut self) {
        match self.edit_fields.get(self.edit_field_cursor).map(|f| f.label) {
            Some("Priority") => self.cycle_priority(-1),
            Some("Color") => self.cycle_color(-1),
            Some("Mode") => self.cycle_mode(-1),
            _ => {}
        }
    }

    fn cycle_mode(&mut self, dir: i32) {
        let current = self.get_field("Mode");
        let n = TIMER_MODES.len() as i32;
        let idx = TIMER_MODES.iter().position(|&m| m == current.as_str()).unwrap_or(0) as i32;
        let next = (idx + dir).rem_euclid(n) as usize;
        if let Some(f) = self.edit_fields.iter_mut().find(|f| f.label == "Mode") {
            f.value = TIMER_MODES[next].to_string();
        }
    }

    fn cycle_priority(&mut self, dir: i32) {
        let current = self.get_field("Priority");
        let n = PRIORITY_OPTIONS.len() as i32;
        let idx = PRIORITY_OPTIONS.iter().position(|&o| o == current.as_str()).unwrap_or(1) as i32;
        let next = (idx + dir).rem_euclid(n) as usize;
        if let Some(f) = self.edit_fields.iter_mut().find(|f| f.label == "Priority") {
            f.value = PRIORITY_OPTIONS[next].to_string();
        }
    }

    fn cycle_color(&mut self, dir: i32) {
        let current = self.get_field("Color");
        let n = TASK_COLORS.len() as i32;
        let idx = TASK_COLORS.iter().position(|(h, _)| *h == current.as_str()).unwrap_or(0) as i32;
        let next = (idx + dir).rem_euclid(n) as usize;
        if let Some(f) = self.edit_fields.iter_mut().find(|f| f.label == "Color") {
            f.value = TASK_COLORS[next].0.to_string();
        }
    }

    pub fn type_char(&mut self, c: char) {
        if let Some(f) = self.edit_fields.get_mut(self.edit_field_cursor) {
            f.value.push(c);
        }
    }

    pub fn edit_backspace(&mut self) {
        if let Some(f) = self.edit_fields.get_mut(self.edit_field_cursor) {
            f.value.pop();
        }
    }

    pub fn save_edit(&mut self) {
        if self.active_panel_type() == Some(PanelType::Timer) {
            let mode_str = self.get_field("Mode");
            let minutes: u64 = self.get_field("Minutes").parse().unwrap_or(25);
            let duration_secs = minutes * 60;
            let id = self.active_panel_id().map(str::to_string);
            if let Some(id) = id {
                let new_state = if mode_str == "Countdown" {
                    TimerState::new_countdown(duration_secs)
                } else {
                    TimerState::new_countup()
                };
                self.timers.insert(id.clone(), new_state);
                if let Some(panel) = self.config.panels.iter_mut().find(|p| p.id == id) {
                    panel.widget = Some(crate::config::TimerWidget {
                        mode: mode_str.to_lowercase(),
                        duration_seconds: duration_secs,
                    });
                }
            }
            self.config.save(&self.config_path).ok();
            self.mode = AppMode::List;
            return;
        }

        match self.active_panel_type() {
            Some(PanelType::Task) => {
                let item = self.fields_to_task();
                if self.is_adding {
                    self.items.tasks.push(item);
                } else if let Some(idx) = self.selected_index()
                    && let Some(t) = self.items.tasks.get_mut(idx) { *t = item; }
            }
            Some(PanelType::Jira) => {
                let item = self.fields_to_jira();
                if self.is_adding {
                    self.items.jira.push(item);
                } else if let Some(idx) = self.selected_index()
                    && let Some(t) = self.items.jira.get_mut(idx) { *t = item; }
            }
            Some(PanelType::GithubPr) => {
                let item = self.fields_to_github_pr();
                if self.is_adding {
                    self.items.github_prs.push(item);
                } else if let Some(idx) = self.selected_index()
                    && let Some(t) = self.items.github_prs.get_mut(idx) { *t = item; }
            }
            Some(PanelType::GithubIssue) => {
                let item = self.fields_to_github_issue();
                if self.is_adding {
                    self.items.github_issues.push(item);
                } else if let Some(idx) = self.selected_index()
                    && let Some(t) = self.items.github_issues.get_mut(idx) { *t = item; }
            }
            _ => {}
        }

        if self.is_adding {
            let new_idx = self.panel_item_count().saturating_sub(1);
            if let Some(id) = self.active_panel_id() {
                self.selected_items.insert(id.to_string(), new_idx);
            }
        }
        self.mode = AppMode::List;

        JsonProvider { path: self.items_path.clone() }.save(&self.items).ok();
    }

    pub fn confirm_delete(&mut self) {
        let idx = match self.selected_index() {
            Some(i) => i,
            None => { self.mode = AppMode::List; return; }
        };

        match self.active_panel_type() {
            Some(PanelType::Task) if idx < self.items.tasks.len() => { self.items.tasks.remove(idx); }
            Some(PanelType::Jira) if idx < self.items.jira.len() => { self.items.jira.remove(idx); }
            Some(PanelType::GithubPr) if idx < self.items.github_prs.len() => { self.items.github_prs.remove(idx); }
            Some(PanelType::GithubIssue) if idx < self.items.github_issues.len() => { self.items.github_issues.remove(idx); }
            _ => {}
        }

        JsonProvider { path: self.items_path.clone() }.save(&self.items).ok();

        let count = self.panel_item_count();
        let id = self.active_panel_id().map(str::to_string);
        if count == 0 {
            if let Some(id) = id { self.selected_items.remove(&id); }
        } else if let Some(id) = id {
            self.selected_items.insert(id, idx.min(count - 1));
        }
        self.mode = AppMode::List;
    }

    pub fn primary_link(&self) -> Option<String> {
        let idx = self.selected_index()?;
        match self.active_panel_type() {
            Some(PanelType::Jira) => {
                self.items.jira.get(idx).map(|t| t.link.clone()).filter(|l| !l.is_empty())
            }
            Some(PanelType::GithubPr) => {
                self.items.github_prs.get(idx).map(|t| t.link.clone()).filter(|l| !l.is_empty())
            }
            Some(PanelType::GithubIssue) => {
                self.items.github_issues.get(idx).map(|t| t.link.clone()).filter(|l| !l.is_empty())
            }
            _ => None,
        }
    }

    pub fn open_link(&self) {
        if let Some(url) = self.primary_link() {
            open::that(url).ok();
        }
    }

    pub fn selected_item_title(&self) -> Option<String> {
        let idx = self.selected_index()?;
        match self.active_panel_type() {
            Some(PanelType::Task) => self.items.tasks.get(idx).map(|t| t.title.clone()),
            Some(PanelType::Jira) => self.items.jira.get(idx).map(|t| t.title.clone()),
            Some(PanelType::GithubPr) => self.items.github_prs.get(idx).map(|t| t.id.clone()),
            Some(PanelType::GithubIssue) => self.items.github_issues.get(idx).map(|t| t.id.clone()),
            _ => None,
        }
    }

    pub fn detail_fields(&self) -> Vec<(&'static str, String)> {
        let idx = match self.selected_index() { Some(i) => i, None => return Vec::new() };
        match self.active_panel_type() {
            Some(PanelType::Task) => {
                if let Some(t) = self.items.tasks.get(idx) {
                    vec![
                        ("Title", t.title.clone()),
                        ("Priority", format!("{:?}", t.priority)),
                        ("Color", t.color.clone()),
                        ("Deadline", t.deadline.clone()),
                        ("Description", t.description.clone()),
                    ]
                } else { Vec::new() }
            }
            Some(PanelType::Jira) => {
                if let Some(t) = self.items.jira.get(idx) {
                    vec![
                        ("ID", t.id.clone()),
                        ("Title", t.title.clone()),
                        ("Link", t.link.clone()),
                        ("Description", t.description.clone()),
                        ("Comment", t.comment.clone()),
                    ]
                } else { Vec::new() }
            }
            Some(PanelType::GithubPr) => {
                if let Some(t) = self.items.github_prs.get(idx) {
                    vec![
                        ("ID", t.id.clone()),
                        ("Link", t.link.clone()),
                        ("Description", t.description.clone()),
                    ]
                } else { Vec::new() }
            }
            Some(PanelType::GithubIssue) => {
                if let Some(t) = self.items.github_issues.get(idx) {
                    vec![
                        ("ID", t.id.clone()),
                        ("Link", t.link.clone()),
                        ("Description", t.description.clone()),
                    ]
                } else { Vec::new() }
            }
            _ => Vec::new(),
        }
    }

    fn get_field(&self, label: &str) -> String {
        self.edit_fields.iter().find(|f| f.label == label).map(|f| f.value.clone()).unwrap_or_default()
    }

    fn fields_to_task(&self) -> Task {
        let priority = match self.get_field("Priority").to_lowercase().as_str() {
            "low" => Priority::Low,
            "high" => Priority::High,
            "critical" => Priority::Critical,
            _ => Priority::Medium,
        };
        let id = if self.is_adding {
            let ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            format!("task-{ms}")
        } else {
            self.selected_index()
                .and_then(|i| self.items.tasks.get(i))
                .map(|t| t.id.clone())
                .unwrap_or_else(|| "task-0".to_string())
        };
        Task {
            id,
            title: self.get_field("Title"),
            description: self.get_field("Description"),
            deadline: self.get_field("Deadline"),
            priority,
            color: self.get_field("Color"),
        }
    }

    fn fields_to_jira(&self) -> JiraItem {
        JiraItem {
            id: self.get_field("ID"),
            title: self.get_field("Title"),
            link: self.get_field("Link"),
            description: self.get_field("Description"),
            comment: self.get_field("Comment"),
        }
    }

    fn fields_to_github_pr(&self) -> GithubPr {
        GithubPr { id: self.get_field("ID"), link: self.get_field("Link"), description: self.get_field("Description") }
    }

    fn fields_to_github_issue(&self) -> GithubIssue {
        GithubIssue { id: self.get_field("ID"), link: self.get_field("Link"), description: self.get_field("Description") }
    }

    fn empty_fields_for_active(&self) -> Vec<EditField> {
        match self.active_panel_type() {
            Some(PanelType::Task) => vec![
                EditField { label: "Title", value: String::new() },
                EditField { label: "Description", value: String::new() },
                EditField { label: "Deadline", value: String::new() },
                EditField { label: "Priority", value: "Medium".to_string() },
                EditField { label: "Color", value: TASK_COLORS[0].0.to_string() },
            ],
            Some(PanelType::Jira) => vec![
                EditField { label: "ID", value: String::new() },
                EditField { label: "Title", value: String::new() },
                EditField { label: "Link", value: String::new() },
                EditField { label: "Description", value: String::new() },
                EditField { label: "Comment", value: String::new() },
            ],
            Some(PanelType::GithubPr) | Some(PanelType::GithubIssue) => vec![
                EditField { label: "ID", value: String::new() },
                EditField { label: "Link", value: String::new() },
                EditField { label: "Description", value: String::new() },
            ],
            _ => Vec::new(),
        }
    }

    fn fields_for_selected(&self) -> Vec<EditField> {
        if self.active_panel_type() == Some(PanelType::Timer) {
            let id = match self.active_panel_id() { Some(id) => id, None => return Vec::new() };
            let timer = match self.timers.get(id) { Some(t) => t, None => return Vec::new() };
            let mode = match timer.mode { TimerMode::Countdown => "Countdown", TimerMode::Countup => "Countup" };
            let minutes = (timer.initial / 60).max(1);
            return vec![
                EditField { label: "Mode", value: mode.to_string() },
                EditField { label: "Minutes", value: minutes.to_string() },
            ];
        }
        let idx = match self.selected_index() { Some(i) => i, None => return Vec::new() };
        match self.active_panel_type() {
            Some(PanelType::Task) => {
                if let Some(t) = self.items.tasks.get(idx) { vec![
                    EditField { label: "Title", value: t.title.clone() },
                    EditField { label: "Description", value: t.description.clone() },
                    EditField { label: "Deadline", value: t.deadline.clone() },
                    EditField { label: "Priority", value: format!("{:?}", t.priority) },
                    EditField { label: "Color", value: t.color.clone() },
                ]} else { Vec::new() }
            }
            Some(PanelType::Jira) => {
                if let Some(t) = self.items.jira.get(idx) { vec![
                    EditField { label: "ID", value: t.id.clone() },
                    EditField { label: "Title", value: t.title.clone() },
                    EditField { label: "Link", value: t.link.clone() },
                    EditField { label: "Description", value: t.description.clone() },
                    EditField { label: "Comment", value: t.comment.clone() },
                ]} else { Vec::new() }
            }
            Some(PanelType::GithubPr) => {
                if let Some(t) = self.items.github_prs.get(idx) { vec![
                    EditField { label: "ID", value: t.id.clone() },
                    EditField { label: "Link", value: t.link.clone() },
                    EditField { label: "Description", value: t.description.clone() },
                ]} else { Vec::new() }
            }
            Some(PanelType::GithubIssue) => {
                if let Some(t) = self.items.github_issues.get(idx) { vec![
                    EditField { label: "ID", value: t.id.clone() },
                    EditField { label: "Link", value: t.link.clone() },
                    EditField { label: "Description", value: t.description.clone() },
                ]} else { Vec::new() }
            }
            _ => Vec::new(),
        }
    }
}
