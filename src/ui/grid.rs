use anyhow::{bail, Result};
use ratatui::layout::Rect;

use crate::config::{Config, Panel, PanelType};

pub struct ResolvedPanel {
    pub id: String,
    pub panel_type: PanelType,
    pub rect: Rect,
}

pub fn resolve(config: &Config, area: Rect) -> Result<Vec<ResolvedPanel>> {
    let cols = config.grid.columns as u16;
    let rows = config.grid.rows as u16;

    if cols == 0 || rows == 0 {
        bail!("grid columns and rows must be greater than 0");
    }
    if cols > 4 || rows > 2 {
        bail!("grid exceeds maximum size of 4 columns × 2 rows");
    }

    validate_panels(config)?;

    let cell_w = area.width / cols;
    let cell_h = area.height / rows;

    let panels = config
        .panels
        .iter()
        .map(|p| {
            let x = area.x + p.cell.col as u16 * cell_w;
            let y = area.y + p.cell.row as u16 * cell_h;
            let w = p.span.col_span as u16 * cell_w;
            let h = p.span.row_span as u16 * cell_h;
            ResolvedPanel {
                id: p.id.clone(),
                panel_type: p.panel_type.clone(),
                rect: Rect::new(x, y, w, h),
            }
        })
        .collect();

    Ok(panels)
}

fn validate_panels(config: &Config) -> Result<()> {
    let cols = config.grid.columns;
    let rows = config.grid.rows;

    for panel in &config.panels {
        let end_col = panel.cell.col + panel.span.col_span;
        let end_row = panel.cell.row + panel.span.row_span;
        if end_col > cols || end_row > rows {
            bail!(
                "panel '{}' exceeds grid bounds ({}×{})",
                panel.id, cols, rows
            );
        }
    }

    for i in 0..config.panels.len() {
        for j in (i + 1)..config.panels.len() {
            if panels_overlap(&config.panels[i], &config.panels[j]) {
                bail!(
                    "panels '{}' and '{}' overlap",
                    config.panels[i].id, config.panels[j].id
                );
            }
        }
    }

    Ok(())
}

fn panels_overlap(a: &Panel, b: &Panel) -> bool {
    let a_col_end = a.cell.col + a.span.col_span;
    let b_col_end = b.cell.col + b.span.col_span;
    let a_row_end = a.cell.row + a.span.row_span;
    let b_row_end = b.cell.row + b.span.row_span;

    a.cell.col < b_col_end
        && b.cell.col < a_col_end
        && a.cell.row < b_row_end
        && b.cell.row < a_row_end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CellPosition, CellSpan, Config, GridConfig, Panel, PanelType};

    fn panel(id: &str, col: u8, row: u8, col_span: u8, row_span: u8) -> Panel {
        Panel {
            id: id.to_string(),
            cell: CellPosition { col, row },
            span: CellSpan { col_span, row_span },
            panel_type: PanelType::Task,
            widget: None,
        }
    }

    fn config(columns: u8, rows: u8, panels: Vec<Panel>) -> Config {
        Config {
            grid: GridConfig { columns, rows },
            panels,
        }
    }

    #[test]
    fn four_equal_panels_2x2() {
        let cfg = config(2, 2, vec![
            panel("p1", 0, 0, 1, 1),
            panel("p2", 1, 0, 1, 1),
            panel("p3", 0, 1, 1, 1),
            panel("p4", 1, 1, 1, 1),
        ]);
        let resolved = resolve(&cfg, Rect::new(0, 0, 100, 40)).unwrap();
        assert_eq!(resolved.len(), 4);
        assert_eq!(resolved[0].rect, Rect::new(0, 0, 50, 20));
        assert_eq!(resolved[1].rect, Rect::new(50, 0, 50, 20));
        assert_eq!(resolved[2].rect, Rect::new(0, 20, 50, 20));
        assert_eq!(resolved[3].rect, Rect::new(50, 20, 50, 20));
    }

    #[test]
    fn col_span_doubles_width() {
        let cfg = config(2, 1, vec![panel("p1", 0, 0, 2, 1)]);
        let resolved = resolve(&cfg, Rect::new(0, 0, 100, 20)).unwrap();
        assert_eq!(resolved[0].rect, Rect::new(0, 0, 100, 20));
    }

    #[test]
    fn row_span_doubles_height() {
        let cfg = config(1, 2, vec![panel("p1", 0, 0, 1, 2)]);
        let resolved = resolve(&cfg, Rect::new(0, 0, 100, 40)).unwrap();
        assert_eq!(resolved[0].rect, Rect::new(0, 0, 100, 40));
    }

    #[test]
    fn panel_offset_in_larger_grid() {
        let cfg = config(4, 2, vec![panel("p1", 2, 1, 2, 1)]);
        let resolved = resolve(&cfg, Rect::new(0, 0, 80, 20)).unwrap();
        assert_eq!(resolved[0].rect, Rect::new(40, 10, 40, 10));
    }

    #[test]
    fn out_of_bounds_col_errors() {
        let cfg = config(2, 2, vec![panel("p1", 2, 0, 1, 1)]);
        assert!(resolve(&cfg, Rect::new(0, 0, 100, 40)).is_err());
    }

    #[test]
    fn out_of_bounds_span_errors() {
        let cfg = config(2, 2, vec![panel("p1", 1, 0, 2, 1)]);
        assert!(resolve(&cfg, Rect::new(0, 0, 100, 40)).is_err());
    }

    #[test]
    fn overlapping_panels_error() {
        let cfg = config(2, 2, vec![
            panel("p1", 0, 0, 2, 1),
            panel("p2", 1, 0, 1, 1),
        ]);
        assert!(resolve(&cfg, Rect::new(0, 0, 100, 40)).is_err());
    }

    #[test]
    fn grid_exceeds_max_columns_errors() {
        let cfg = config(5, 2, vec![]);
        assert!(resolve(&cfg, Rect::new(0, 0, 100, 40)).is_err());
    }

    #[test]
    fn grid_exceeds_max_rows_errors() {
        let cfg = config(4, 3, vec![]);
        assert!(resolve(&cfg, Rect::new(0, 0, 100, 40)).is_err());
    }
}
