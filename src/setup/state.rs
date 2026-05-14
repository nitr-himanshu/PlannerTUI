use crate::config::PanelType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetupStep {
    Welcome,
    FeatureSelect,
    LayoutPreview,
    ConfigGuide,
    ItemsGuide,
    SkillGuide,
    Complete,
}

pub struct FeatureOption {
    pub label: &'static str,
    pub short: &'static str,
    pub description: &'static str,
    pub panel_type: PanelType,
    pub selected: bool,
}

pub struct SetupState {
    pub step: SetupStep,
    pub features: Vec<FeatureOption>,
    pub cursor: usize,
    pub layout_option: usize,
    pub no_selection_hint: bool,
}

impl SetupState {
    pub fn new() -> Self {
        Self {
            step: SetupStep::Welcome,
            features: vec![
                FeatureOption {
                    label: "Tasks",
                    short: "Tasks",
                    description: "Personal tasks with title, due date, priority and color",
                    panel_type: PanelType::Task,
                    selected: false,
                },
                FeatureOption {
                    label: "GitHub PRs",
                    short: "GH PRs",
                    description: "Track open pull requests assigned to you",
                    panel_type: PanelType::GithubPr,
                    selected: false,
                },
                FeatureOption {
                    label: "GitHub Issues",
                    short: "Issues",
                    description: "Track open issues assigned to you",
                    panel_type: PanelType::GithubIssue,
                    selected: false,
                },
                FeatureOption {
                    label: "JIRA",
                    short: "JIRA",
                    description: "View JIRA tickets matching your filter",
                    panel_type: PanelType::Jira,
                    selected: false,
                },
                FeatureOption {
                    label: "Timer",
                    short: "Timer",
                    description: "Countdown or countup focus timer",
                    panel_type: PanelType::Timer,
                    selected: false,
                },
            ],
            cursor: 0,
            layout_option: 0,
            no_selection_hint: false,
        }
    }

    pub fn selected_count(&self) -> usize {
        self.features.iter().filter(|f| f.selected).count()
    }

    pub fn layout_options_count(&self) -> usize {
        match self.selected_count() {
            0..=2 => 1,
            _ => 2,
        }
    }

    pub fn next_feature(&mut self) {
        self.no_selection_hint = false;
        if self.cursor + 1 < self.features.len() {
            self.cursor += 1;
        }
    }

    pub fn prev_feature(&mut self) {
        self.no_selection_hint = false;
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn toggle_feature(&mut self) {
        self.no_selection_hint = false;
        if let Some(f) = self.features.get_mut(self.cursor) {
            f.selected = !f.selected;
        }
    }

    pub fn next_layout(&mut self) {
        if self.layout_option + 1 < self.layout_options_count() {
            self.layout_option += 1;
        }
    }

    pub fn prev_layout(&mut self) {
        self.layout_option = self.layout_option.saturating_sub(1);
    }

    pub fn advance(&mut self) -> bool {
        match self.step {
            SetupStep::FeatureSelect if self.selected_count() == 0 => {
                self.no_selection_hint = true;
                false
            }
            _ => {
                self.step = match self.step {
                    SetupStep::Welcome => SetupStep::FeatureSelect,
                    SetupStep::FeatureSelect => SetupStep::LayoutPreview,
                    SetupStep::LayoutPreview => SetupStep::ConfigGuide,
                    SetupStep::ConfigGuide => SetupStep::ItemsGuide,
                    SetupStep::ItemsGuide => SetupStep::SkillGuide,
                    SetupStep::SkillGuide => SetupStep::Complete,
                    SetupStep::Complete => SetupStep::Complete,
                };
                true
            }
        }
    }
}
