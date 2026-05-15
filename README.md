# PlannerTUI

[![CI](https://github.com/nitr-himanshu/PlannerTUI/actions/workflows/ci.yml/badge.svg)](https://github.com/nitr-himanshu/PlannerTUI/actions/workflows/ci.yml)

A customizable, Rust-based terminal dashboard for managing tasks, JIRA issues, GitHub items, and focus timers — all in one configurable grid.

---

## Features

- **Setup wizard** — interactive first-run guide that configures your layout based on what you need
- **Grid dashboard** — up to 4 columns × 2 rows; panels are fully configurable and cells can be merged
- **5 panel types** — Tasks, JIRA, GitHub PRs, GitHub Issues, Timer
- **Full CRUD** — add, edit, delete items directly from the list view; read full detail with `Enter`
- **Task form** — priority picker (Low / Medium / High / Critical), 10 predefined dark-mode color swatches
- **Timer widget** — live countdown or countup, vertically centered big-text display, editable duration
- **Clickable links** — press `o` in any detail view to open the URL in your default browser
- **Mouse support** — click to focus a panel, scroll wheel to navigate items
- **Status bar** — context-aware keybinding hints that change per mode
- **AI sync skill** — populate `items.json` automatically via Claude + MCP (no API tokens needed)
- **JSON-backed** — human-editable `config.json` and `items.json`, no database required
- **Cross-platform** — Windows and Linux

---

## Quick Start

1. Download the binary for your platform from the [Releases](https://github.com/nitr-himanshu/PlannerTUI/releases) page
2. Run it — on first launch an interactive wizard guides you through setup:
   - Choose which panels to enable (Tasks, GitHub PRs, GitHub Issues, JIRA, Timer)
   - Pick a suggested grid layout
   - Learn how to edit `config.json`, `items.json`, and the AI sync skill
3. Your dashboard launches automatically when the wizard finishes

Files created next to the executable:

```text
<executable-dir>/
└── .planner_tui/
    ├── config.json   # grid layout and panel assignments
    └── items.json    # your tasks, JIRA items, and GitHub refs
```

---

## Keybindings

### List view (default)

| Key | Action |
| --- | --- |
| `Tab` / `Shift+Tab` | Move focus to next / previous panel |
| `↑` `↓` | Move selection cursor within the focused panel |
| `Enter` | Open read-only detail view for the selected item |
| `a` | Add a new item to the focused panel |
| `e` | Edit the selected item (or edit timer settings) |
| `d` | Delete the selected item (with confirmation) |
| `Space` | Start / pause the focused timer |
| `r` | Reset the focused timer |
| `q` | Quit |

Mouse: click any panel to focus it; scroll wheel moves the selection cursor.

### Detail view (read-only)

| Key | Action |
| --- | --- |
| `o` | Open the item's link in the default browser |
| `Esc` | Back to list |

### Edit / Add dialog

| Key | Action |
| --- | --- |
| `↑` `↓` / `Tab` | Move between fields |
| `←` `→` | Cycle options on Priority, Color, and Mode fields |
| `Ctrl+S` | Save |
| `Esc` | Cancel |

---

## Item Types

### Task

| Field | Notes |
| --- | --- |
| Title | Task name |
| Description | Optional details |
| Deadline | ISO 8601 date-time (e.g. `2026-05-20T18:00:00`) |
| Priority | `Low` · `Medium` · `High` · `Critical` — color-coded picker |
| Color | Choose from 10 dark-mode swatches (Coral, Amber, Yellow, Sky, Mint, Blue, Lavender, Pink, Teal, Gold) |

Task IDs are auto-generated on creation.

### JIRA

| Field | Notes |
| --- | --- |
| ID | Issue key, e.g. `PROJ-42` |
| Title | Issue summary |
| Link | URL — press `o` in detail view to open |
| Description | Short description |
| Comment | Latest comment |

### GitHub PR / Issue

| Field | Notes |
| --- | --- |
| ID | e.g. `org/repo#123` |
| Link | URL — press `o` in detail view to open |
| Description | Optional notes |

### Timer

Editable fields (press `e`):

| Field | Notes |
| --- | --- |
| Mode | `Countdown` or `Countup` |
| Minutes | Duration for countdown mode |

The timer renders as large block characters that scale to the panel size. Timer settings are saved to `config.json`.

---

## Layout Configuration (`config.json`)

```json
{
  "grid": { "columns": 3, "rows": 2 },
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
      "widget": { "mode": "countdown", "duration_seconds": 1500 }
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
- `span` — how many columns/rows the panel occupies (merged cells)
- `type` — `task` · `jira` · `github_pr` · `github_issue` · `timer`
- Maximum grid: 4 columns × 2 rows

---

## Items (`items.json`)

```json
{
  "tasks": [
    {
      "id": "task-1713000000000",
      "title": "Implement login flow",
      "description": "OAuth2 via GitHub",
      "deadline": "2026-05-20T18:00:00",
      "priority": "High",
      "color": "#FF6B6B"
    }
  ],
  "jira": [
    {
      "id": "PROJ-42",
      "title": "API rate limiting",
      "link": "https://your-org.atlassian.net/browse/PROJ-42",
      "description": "Implement rate limiting on public endpoints",
      "comment": "Token bucket algorithm proposed"
    }
  ],
  "github_prs": [
    {
      "id": "org/repo#123",
      "link": "https://github.com/org/repo/pull/123",
      "description": ""
    }
  ],
  "github_issues": [
    {
      "id": "org/repo#456",
      "link": "https://github.com/org/repo/issues/456",
      "description": ""
    }
  ]
}
```

---

## AI Sync Skill

Populate `items.json` automatically using Claude and your connected MCP servers — no API tokens or credentials needed.

1. Open [`docs/sync-skill.md`](docs/sync-skill.md) in Claude
2. Say: **"Follow the sync skill"**
3. Claude detects your GitHub / JIRA MCP connections, asks for filters, fetches live data, and writes `items.json`

Re-run any time to refresh your dashboard.

---

## Project Structure

```text
src/
├── main.rs               # entry point and async run loop
├── app.rs                # global app state and all CRUD logic
├── config/
│   ├── mod.rs            # Config struct, loader/saver
│   └── defaults.rs       # default config and sample items
├── model/
│   ├── task.rs           # Task { id, title, description, deadline, priority, color }
│   ├── jira.rs           # JiraItem { id, title, link, description, comment }
│   └── github.rs         # GithubPr, GithubIssue { id, link, description }
├── storage/
│   ├── mod.rs            # DataProvider trait
│   └── json.rs           # JSON file backend
├── setup/                # first-run interactive wizard
├── ui/
│   ├── mod.rs            # top-level render dispatcher
│   ├── grid.rs           # grid layout engine (resolves panel Rects)
│   ├── status_bar.rs     # context-aware bottom bar
│   ├── detail.rs         # read-only item detail overlay
│   ├── dialog.rs         # add/edit/delete confirmation dialogs
│   └── panel/
│       ├── task.rs
│       ├── jira.rs
│       ├── github_pr.rs
│       ├── github_issue.rs
│       └── timer.rs      # big-text timer with dynamic pixel size
├── widget/
│   └── timer.rs          # TimerState — tick, toggle, reset, format
└── event/
    ├── mod.rs            # AppEvent enum
    ├── keyboard.rs       # mode-aware keyboard handler
    └── mouse.rs          # click-to-focus, scroll (gated by AppMode)
```

---

## Building from Source

**Prerequisites:** Rust stable toolchain ([rustup.rs](https://rustup.rs))

```bash
git clone https://github.com/nitr-himanshu/PlannerTUI
cd PlannerTUI
cargo build --release
```

Binary: `target/release/planner_tui` (Linux) · `target/release/planner_tui.exe` (Windows)

---

## Dependencies

| Crate | Purpose |
| --- | --- |
| `ratatui` | TUI rendering framework |
| `crossterm` | Cross-platform terminal backend |
| `tui-big-text` | Large block-character timer display |
| `serde` + `serde_json` | JSON serialization |
| `chrono` | Deadline formatting and timestamps |
| `tokio` | Async runtime for timer ticks and event handling |
| `open` | Opens URLs in the default browser |
| `anyhow` | Error handling |

---

## Roadmap

| Feature | Status |
| --- | --- |
| Setup wizard | Done |
| Grid layout + cell merging | Done |
| Task, JIRA, GitHub panel types | Done |
| Full CRUD (add, edit, delete, detail view) | Done |
| Timer widget with big-text display | Done |
| Mouse support | Done |
| Clickable links (`o` to open) | Done |
| AI sync skill (Claude MCP) | Done |
| GitHub Actions release workflow | Done |
| SQLite backend | Future |
| JIRA REST API provider | Future |
| GitHub REST API provider | Future |

---

## License

GNU General Public License v3 — see [LICENSE](LICENSE).
