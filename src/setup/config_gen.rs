use crate::config::{CellPosition, CellSpan, Config, GridConfig, Panel, PanelType, TimerWidget};

use super::state::SetupState;

pub fn generate(state: &SetupState) -> Config {
    let types: Vec<PanelType> = state
        .features
        .iter()
        .filter(|f| f.selected)
        .map(|f| f.panel_type)
        .collect();

    match types.len() {
        1 => layout_1x1(&types),
        2 => layout_2x1(&types),
        3 if state.layout_option == 1 => layout_tall_left_3(&types),
        3 => layout_3x1(&types),
        4 if state.layout_option == 1 => layout_3x2_asymmetric(&types),
        4 => layout_2x2(&types),
        5 if state.layout_option == 1 => layout_3x2_five_alt(&types),
        5 => layout_3x2_five(&types),
        _ => layout_3x2_five(&types[..5]),
    }
}

fn panel(id: &str, pt: PanelType, col: u8, row: u8, col_span: u8, row_span: u8) -> Panel {
    Panel {
        id: id.to_string(),
        cell: CellPosition { col, row },
        span: CellSpan { col_span, row_span },
        panel_type: pt,
        widget: (pt == PanelType::Timer).then(|| TimerWidget {
            mode: "countdown".to_string(),
            duration_seconds: 1500,
        }),
    }
}

fn layout_1x1(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 1, rows: 1 },
        panels: vec![panel("panel-1", t[0], 0, 0, 1, 1)],
    }
}

fn layout_2x1(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 2, rows: 1 },
        panels: vec![panel("panel-1", t[0], 0, 0, 1, 1), panel("panel-2", t[1], 1, 0, 1, 1)],
    }
}

fn layout_tall_left_3(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 2, rows: 2 },
        panels: vec![
            panel("panel-1", t[0], 0, 0, 1, 2),
            panel("panel-2", t[1], 1, 0, 1, 1),
            panel("panel-3", t[2], 1, 1, 1, 1),
        ],
    }
}

fn layout_3x1(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 3, rows: 1 },
        panels: vec![
            panel("panel-1", t[0], 0, 0, 1, 1),
            panel("panel-2", t[1], 1, 0, 1, 1),
            panel("panel-3", t[2], 2, 0, 1, 1),
        ],
    }
}

fn layout_2x2(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 2, rows: 2 },
        panels: vec![
            panel("panel-1", t[0], 0, 0, 1, 1),
            panel("panel-2", t[1], 1, 0, 1, 1),
            panel("panel-3", t[2], 0, 1, 1, 1),
            panel("panel-4", t[3], 1, 1, 1, 1),
        ],
    }
}

fn layout_3x2_five(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 3, rows: 2 },
        panels: vec![
            panel("panel-1", t[0], 0, 0, 1, 1),
            panel("panel-2", t[1], 1, 0, 1, 1),
            panel("panel-3", t[2], 2, 0, 1, 1),
            panel("panel-4", t[3], 0, 1, 1, 1),
            panel("panel-5", t[4], 1, 1, 2, 1),
        ],
    }
}

fn layout_3x2_five_alt(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 3, rows: 2 },
        panels: vec![
            panel("panel-1", t[0], 0, 0, 2, 1),
            panel("panel-2", t[1], 2, 0, 1, 1),
            panel("panel-3", t[2], 0, 1, 1, 1),
            panel("panel-4", t[3], 1, 1, 1, 1),
            panel("panel-5", t[4], 2, 1, 1, 1),
        ],
    }
}

fn layout_3x2_asymmetric(t: &[PanelType]) -> Config {
    Config {
        grid: GridConfig { columns: 3, rows: 2 },
        panels: vec![
            panel("panel-1", t[0], 0, 0, 2, 1),
            panel("panel-2", t[1], 2, 0, 1, 1),
            panel("panel-3", t[2], 0, 1, 1, 1),
            panel("panel-4", t[3], 1, 1, 2, 1),
        ],
    }
}
