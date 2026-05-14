# PlannerTUI — Requirements

## Overview

PlannerTUI is a Rust-based, customizable terminal user interface (TUI) application for managing tasks and project items across multiple domains (JIRA, GitHub, personal tasks). Users configure a grid layout and assign item types or widgets to each cell.

---

## UI Layout

### Grid

- Maximum layout: **4 columns × 2 rows** (8 panels)
- Users can configure any layout within that boundary (e.g., 1×1, 2×1, 3×2)
- **Cell merging**: adjacent cells can be merged to form a larger panel (e.g., merging 2 cells horizontally gives a wide panel spanning 2 columns)
- Each panel is independently configurable — any item type or widget can be placed in any cell
- Grid and merge configuration is defined by the user (e.g., in a JSON config file)

### Mouse Support

- Mouse clicks to select and focus a panel
- Scroll within a panel to navigate content
- Click interactive elements (links, buttons) within panels

### Status Bar

- Fixed bar at the **bottom** of the terminal
- Displays contextual information: current mode, active panel, keybindings hint, time, etc.

---

## Item Types

Each panel can be configured to display one of the following item types.

### JIRA Item

| Field       | Description                     |
| ----------- | ------------------------------- |
| Title       | Issue summary / title           |
| Link        | URL to the JIRA issue           |
| Description | Short description of the issue  |
| Comment     | Latest or selected comment      |

### Task

| Field    | Description                                           |
| -------- | ----------------------------------------------------- |
| Title    | Task name                                             |
| Deadline | Due date/time                                         |
| Priority | Priority level (e.g., Low / Medium / High / Critical) |
| Color    | User-assigned color label for visual grouping         |

### GitHub PR

| Field   | Description               |
| ------- | ------------------------- |
| PR Link | URL to the pull request   |

### GitHub Issue

| Field       | Description               |
| ----------- | ------------------------- |
| Issue Link  | URL to the GitHub issue   |

---

## Widgets

Widgets are interactive or informational components that can be placed in any grid cell.

### Timer

- Counts up or down (user-configurable)
- Can be set by the user at runtime
- Placeable in any grid cell

---

## File System Layout

On first run, the executable automatically creates a `.planner_tui/` folder in the **same directory as the executable**. Two files are initialized inside it if they do not already exist:

```text
<executable-dir>/
└── .planner_tui/
    ├── config.json   # grid layout, panel assignments, widget settings
    └── items.json     # stored items (tasks, JIRA entries, GitHub refs, etc.)
```

- If `.planner_tui/` or either file is missing at startup, the application creates them with sensible defaults.
- Both files are human-editable JSON.

### config.json — Sample

```json
{
  "grid": {
    "columns": 3,
    "rows": 2
  },
  "panels": [
    {
      "id": "panel-1",
      "cell": { "col": 0, "row": 0 },
      "span": { "col_span": 2, "row_span": 1 },
      "type": "task"
    },
    {
      "id": "panel-2",
      "cell": { "col": 2, "row": 0 },
      "span": { "col_span": 1, "row_span": 2 },
      "type": "jira"
    },
    {
      "id": "panel-3",
      "cell": { "col": 0, "row": 1 },
      "span": { "col_span": 1, "row_span": 1 },
      "type": "timer",
      "widget": {
        "mode": "countdown",
        "duration_seconds": 1500
      }
    },
    {
      "id": "panel-4",
      "cell": { "col": 1, "row": 1 },
      "span": { "col_span": 1, "row_span": 1 },
      "type": "github_pr"
    }
  ]
}
```

- `cell` — zero-indexed top-left origin of the panel
- `span` — how many columns/rows the panel occupies (cell merging)
- `type` — one of `task`, `jira`, `github_pr`, `github_issue`, `timer`
- `widget` — optional block present only for `timer` panels

### items.json — Sample

```json
{
  "tasks": [
    {
      "id": "task-1",
      "title": "Implement login flow",
      "deadline": "2026-05-20T18:00:00",
      "priority": "High",
      "color": "#FF6B6B"
    }
  ],
  "jira": [
    {
      "id": "jira-1",
      "title": "API rate limiting",
      "link": "https://your-org.atlassian.net/browse/PROJ-42",
      "description": "Implement rate limiting on all public endpoints",
      "comment": "Will use token bucket algorithm — see RFC in Confluence"
    }
  ],
  "github_prs": [
    {
      "id": "pr-1",
      "link": "https://github.com/your-org/your-repo/pull/123"
    }
  ],
  "github_issues": [
    {
      "id": "issue-1",
      "link": "https://github.com/your-org/your-repo/issues/456"
    }
  ]
}
```

- `priority` — one of `Low`, `Medium`, `High`, `Critical`
- `color` — hex color string for visual grouping of tasks
- `deadline` — ISO 8601 date-time string

---

## Data Providers

| Provider          | Status                  | Notes                                |
| ----------------- | ----------------------- | ------------------------------------ |
| JSON file         | **v1 — primary target** | `items.json` inside `.planner_tui/`  |
| Database (SQLite) | Future                  | Persistent local DB backend          |
| JIRA REST API     | Future                  | Live fetch and update of JIRA items  |
| GitHub REST API   | Future                  | Live fetch of PRs and issues         |

The data provider is abstracted so that v1 (JSON) can be swapped for future backends without changing the UI layer.

---

## Platform Support

| Platform | Support                    |
| -------- | -------------------------- |
| Windows  | Primary development target |
| Linux    | Supported build target     |

The project must compile and run correctly on both platforms. CI should validate builds for both targets.

---

## Technology

- **Language**: Rust (stable toolchain)
- **Build system**: Cargo
- **TUI framework**: TBD (e.g., `ratatui` recommended)
- **Configuration**: JSON (v1)

---

## Out of Scope (v1)

- Remote API integrations (JIRA, GitHub)
- Database backend
- Theming / custom color schemes beyond per-task color labels
