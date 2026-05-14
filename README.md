# PlannerTUI

A customizable, Rust-based terminal user interface for managing tasks, JIRA issues, and GitHub items — all in one configurable grid dashboard.

---

## Features

- **Grid dashboard** — up to 4 columns × 2 rows, fully configurable
- **Cell merging** — merge adjacent cells to create wider or taller panels
- **Multiple item types** — Tasks, JIRA issues, GitHub PRs, GitHub Issues
- **Timer widget** — countdown or countup, placeable in any cell
- **Mouse support** — click to focus panels, scroll content, click links
- **Status bar** — active panel, mode, clock, keybinding hints
- **JSON-backed** — human-editable `config.json` and `items.json`, no database required
- **Cross-platform** — Windows and Linux

---

## Quick Start

1. Download the binary for your platform from the Releases page
2. Run it — on first launch it creates `.planner_tui/` next to the executable:

   ```text
   <executable-dir>/
   └── .planner_tui/
       ├── config.json   # grid layout and panel assignments
       └── items.json    # your tasks, JIRA items, and GitHub refs
   ```

3. Edit `config.json` to set your layout, then edit `items.json` to add your items
4. Re-run the binary — your dashboard loads automatically

---

## Layout Configuration (`config.json`)

The grid supports up to **4 columns × 2 rows**. Panels are zero-indexed from the top-left.
Set `col_span` / `row_span` greater than 1 to merge cells.

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

**Panel types:** `task` · `jira` · `github_pr` · `github_issue` · `timer`

---

## Items (`items.json`)

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
      "id": "PROJ-42",
      "title": "API rate limiting",
      "link": "https://your-org.atlassian.net/browse/PROJ-42",
      "description": "Implement rate limiting on all public endpoints",
      "comment": "Will use token bucket algorithm"
    }
  ],
  "github_prs": [
    {
      "id": "org/repo#123",
      "link": "https://github.com/org/repo/pull/123"
    }
  ],
  "github_issues": [
    {
      "id": "org/repo#456",
      "link": "https://github.com/org/repo/issues/456"
    }
  ]
}
```

**Priority values:** `Low` · `Medium` · `High` · `Critical`

**Color:** any hex color string, e.g. `#FF6B6B`

---

## Syncing Items with AI (Sync Skill)

If you use Claude with MCP servers (GitHub MCP, JIRA MCP), you can populate `items.json`
automatically without setting up any API tokens.

Open [`docs/sync-skill.md`](docs/sync-skill.md) in Claude and say: **"Follow the sync skill."**

Claude will detect your connected MCP services, ask for filter conditions (repos, JQL query, etc.),
fetch the data, and write `items.json` for you. Re-run any time to refresh.

---

## Keybindings

| Key | Action |
| --- | --- |
| `Tab` / `Shift+Tab` | Move focus to next / previous panel |
| `↑` `↓` | Scroll content within the focused panel |
| `q` | Quit |
| `Space` | Start / pause timer (when timer panel is focused) |
| `r` | Reset timer |

Mouse is also supported — click any panel to focus it, scroll to navigate content.

---

## Project Structure

```text
src/
├── main.rs               # entry point and run loop
├── app.rs                # global app state
├── config/
│   ├── mod.rs            # Config struct, loader/saver
│   └── defaults.rs       # default config.json generation
├── model/
│   ├── task.rs           # Task struct
│   ├── jira.rs           # JiraItem struct
│   └── github.rs         # GithubPr, GithubIssue structs
├── storage/
│   ├── mod.rs            # DataProvider trait
│   └── json.rs           # JSON file backend (v1)
├── ui/
│   ├── mod.rs            # top-level render entry
│   ├── grid.rs           # grid layout engine
│   ├── status_bar.rs     # bottom status bar
│   └── panel/
│       ├── task.rs
│       ├── jira.rs
│       ├── github_pr.rs
│       ├── github_issue.rs
│       └── timer.rs
├── widget/
│   └── timer.rs          # timer state and tick logic
└── event/
    ├── mod.rs            # event loop
    ├── keyboard.rs       # keyboard handler
    └── mouse.rs          # mouse handler
```

---

## Building from Source

**Prerequisites:** Rust stable toolchain ([rustup.rs](https://rustup.rs))

```bash
git clone https://github.com/your-org/PlannerTUI
cd PlannerTUI
cargo build --release
```

Binary output: `target/release/planner_tui` (Linux) or `target/release/planner_tui.exe` (Windows)

---

## Dependencies

| Crate | Purpose |
| --- | --- |
| `ratatui` | TUI rendering framework |
| `crossterm` | Cross-platform terminal backend |
| `serde` + `serde_json` | JSON serialization |
| `chrono` | Deadline formatting and timer |
| `anyhow` | Error handling |
| `tokio` | Async runtime for timer ticks |

---

## Roadmap

| Feature | Status |
| --- | --- |
| Grid layout + cell merging | Planned |
| Task, JIRA, GitHub panel types | Planned |
| Timer widget | Planned |
| Mouse support | Planned |
| JSON data backend | Planned |
| AI sync skill (Claude MCP) | Done |
| SQLite backend | Future |
| JIRA REST API provider | Future |
| GitHub REST API provider | Future |

---

## License

GNU General Public License v3 — see [LICENSE](LICENSE).
