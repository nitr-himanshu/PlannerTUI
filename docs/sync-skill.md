# Sync Skill — PlannerTUI items.json Updater

## Purpose

This skill instructs Claude to populate `items.json` using data from
services already connected via MCP. No API tokens or credentials are required —
Claude uses the MCP tools already available in your session.

Run this skill by opening it and saying: **"Follow the sync skill."**

---

## How It Works

1. Claude checks which MCP tools are active in the current session
2. Claude asks for your filter conditions per data source (one at a time, iteratively)
3. Claude asks where `planner.exe` is located and locates `items.json`
4. Claude asks where to save the refresh skill
5. Claude fetches the data and writes it into `items.json`
6. Claude saves your filters as a **one-click refresh skill** — run `/refresh-planner` any time to re-sync without re-entering filters

The first run sets everything up. Every run after that is a single command.

---

## Instructions for AI

Follow every step in order. Do not skip steps or batch questions unless explicitly told to.

---

### Step 1 — Detect Available MCP Data Sources

Inspect the list of tools currently available to you. Look for MCP tools matching these categories:

| Category      | Look for tool names containing      | maps to items.json key |
| ------------- | ----------------------------------- | ---------------------- |
| GitHub PRs    | `github` + (`pull_request` or `pr`) | `github_prs`           |
| GitHub Issues | `github` + `issue`                  | `github_issues`        |
| JIRA          | `jira` + (`search` or `issue`)      | `jira`                 |

Report back to the user which categories are available and which are not.
If none of the above are found, stop and tell the user which MCP servers to install.

---

### Step 2 — Collect GitHub Repositories (if GitHub MCP is available)

If GitHub MCP is detected, collect repos one at a time:

1. Ask: **"Which GitHub repo should I watch? (e.g. `org/repo`)"**
2. After the user provides one, ask: **"Do you want to add another repo?"**
3. Repeat until the user says no.

Also ask once (not per repo):

- **"Should I limit GitHub PRs to ones assigned to you?"** (yes/no)
- **"Should I limit GitHub Issues to ones assigned to you?"** (yes/no)
- **"Any label filters for GitHub Issues?"** (comma-separated, or none)

Save all collected repos and filter settings — you will use them in Step 5.

---

### Step 3 — Collect JIRA Projects and Filters (if JIRA MCP is available)

If JIRA MCP is detected, collect projects one at a time:

1. Ask: **"Which JIRA project key should I pull from? (e.g. `PROJ`)"**
2. After the user provides one, ask: **"Do you want to add another JIRA project?"**
3. Repeat until the user says no.

Then ask these filter questions **once** (apply to all collected projects):

- **"Show only issues in the active sprint?"** (yes/no)
- **"Show only issues assigned to you (current user)?"** (yes/no)
- **"Show only open issues (exclude Done/Closed/Resolved)?"** (yes/no)

Save all collected project keys and filter answers — you will use them in Step 5.

---

### Step 4 — Locate planner.exe and items.json

Ask the user: **"Where is `planner.exe` located? (provide the full path or directory)"**

Once the user provides the path:

- Resolve the directory containing `planner.exe`
- Look for `items.json` in that same directory
- If not found there, look for it inside a `.planner_tui/` subdirectory within that directory
- If still not found, tell the user and ask them to confirm the correct path to `items.json`

Save the resolved absolute path to `items.json` — you will use it in Steps 5 and 6 and must embed it in the refresh skill.

---

### Step 5 — Ask Where to Save the Refresh Skill

Ask the user:
**"Where should I save the refresh skill file? Press Enter to use the default, or provide a custom path."**

Show the default clearly:
> Default: `~/.claude/commands/refresh-planner.md`  
> (on Windows this is `%USERPROFILE%\.claude\commands\refresh-planner.md`)

If the user provides a custom path, use that. Otherwise use the default.
Create the directory if it does not exist.

Save the resolved path — you will write the refresh skill there in Step 7.

---

### Step 6 — Fetch Data

Using the MCP tools detected in Step 1, the filters from Steps 2–3, and the `items.json` path from Step 4, fetch the data now.

#### GitHub PRs

For each repo collected in Step 2, call the GitHub MCP tool to list pull requests.
Apply the assignee filter if the user said yes.

Transform each result to:

```json
{
  "id": "<org/repo>#<number>",
  "link": "<html_url>"
}
```

#### GitHub Issues

For each repo collected in Step 2, call the GitHub MCP tool to list issues.
Exclude items that have a `pull_request` field (those are PRs, not issues).
Apply label and assignee filters if specified.

Transform each result to:

```json
{
  "id": "<org/repo>#<number>",
  "link": "<html_url>"
}
```

#### JIRA

For each project key collected in Step 3, build a JQL query using the filters:

- Base: `project = <KEY>`
- If active sprint only: add `AND sprint in openSprints()`
- If assigned to me only: add `AND assignee = currentUser()`
- If open only: add `AND status NOT IN (Done, Closed, Resolved)`

Call the JIRA MCP tool with the constructed JQL.

Transform each result to:

```json
{
  "id": "<issue-key>",
  "title": "<summary>",
  "link": "<issue-url>",
  "description": "<short description, one sentence max>",
  "comment": "<most recent comment, or empty string>"
}
```

---

### Step 7 — Read Existing items.json

Read `items.json` at the path resolved in Step 4. If missing or unreadable, use this default:

```json
{
  "tasks": [],
  "jira": [],
  "github_prs": [],
  "github_issues": []
}
```

---

### Step 8 — Merge and Write

Apply these rules:

- **`tasks`** — never touch. Preserve exactly as-is.
- **`jira`** — replace with fresh data from Step 6 (JIRA). If JIRA MCP was unavailable, leave unchanged.
- **`github_prs`** — replace with fresh data from Step 6. If GitHub MCP was unavailable, leave unchanged.
- **`github_issues`** — replace with fresh data from Step 6. If GitHub MCP was unavailable, leave unchanged.

Write the merged result back to `items.json` (the path from Step 4) with 2-space indentation.

---

### Step 9 — Report

Tell the user:

- The absolute path where `items.json` was written
- How many items were written per category
- Which categories were skipped (MCP not available) and which MCP to add to enable them

---

### Step 10 — Save the Refresh Skill

Write the refresh skill file to the path determined in Step 5 (create the directory if needed).

The file must be self-contained — all filters pre-filled, no questions asked at runtime.
Replace every `<placeholder>` with actual values collected during this session.

````markdown
# PlannerTUI — Refresh Data

Fetch the latest data from connected MCP sources and update items.json.

## Configuration (pre-configured — do not ask the user)

```yaml
items_json_path: "<absolute path to items.json from Step 4>"

github:
  prs:
    repos:
      - "<org/repo>"
      # add more repos here if collected
    assigned_to_me: <true|false>
  issues:
    repos:
      - "<org/repo>"
      # add more repos here if collected
    assigned_to_me: <true|false>
    labels: [<label1>, <label2>]   # empty list if none

jira:
  projects:
    - "<PROJECT_KEY>"
    # add more project keys here if collected
  active_sprint_only: <true|false>
  assigned_to_me: <true|false>
  open_only: <true|false>
```

## Instructions for AI

Follow every step in order. Do not ask for any configuration — it is set above.

1. **Fetch GitHub PRs** (if GitHub MCP available): for each repo in `github.prs.repos`,
   list pull requests. Apply `assigned_to_me` filter. Transform to `{"id": "org/repo#N", "link": "..."}`.

2. **Fetch GitHub Issues** (if GitHub MCP available): for each repo in `github.issues.repos`,
   list issues (exclude items with a `pull_request` field). Apply label and `assigned_to_me` filters.
   Transform to `{"id": "org/repo#N", "link": "..."}`.

3. **Fetch JIRA** (if JIRA MCP available): for each key in `jira.projects`, build JQL:
   - Base: `project = <KEY>`
   - If `active_sprint_only`: add `AND sprint in openSprints()`
   - If `assigned_to_me`: add `AND assignee = currentUser()`
   - If `open_only`: add `AND status NOT IN (Done, Closed, Resolved)`
   Call the JIRA MCP tool. Transform to `{"id": "...", "title": "...", "link": "...", "description": "...", "comment": "..."}`.

4. **Read** `items.json` at `items_json_path`. Preserve `tasks` exactly.
   Replace `jira`, `github_prs`, `github_issues` with fresh data (skip if MCP unavailable).

5. **Write** merged result back to `items_json_path` with 2-space indentation.

6. **Report** how many items were written per category and the file path updated.
````

After writing the file, tell the user:

> **Refresh skill saved.**
> Run **`/refresh-planner`** any time to re-sync your dashboard — no configuration needed.
>
> Skill saved at: `<path from Step 5>`  
> items.json path: `<path from Step 4>`  
> Edit the skill file to change filters.

---

## items.json Schema Reference

```json
{
  "tasks": [
    {
      "id": "task-1",
      "title": "string",
      "deadline": "ISO-8601 or empty string",
      "priority": "Low | Medium | High | Critical",
      "color": "#RRGGBB"
    }
  ],
  "jira": [
    {
      "id": "PROJ-42",
      "title": "string",
      "link": "https://...",
      "description": "string",
      "comment": "string"
    }
  ],
  "github_prs": [
    {
      "id": "org/repo#123",
      "link": "https://github.com/..."
    }
  ],
  "github_issues": [
    {
      "id": "org/repo#456",
      "link": "https://github.com/..."
    }
  ]
}
```

---

## Recommended MCP Servers

| Data Source | MCP Server to install                          |
| ----------- | ---------------------------------------------- |
| GitHub      | `github` MCP (official GitHub MCP server)      |
| JIRA        | `jira` MCP (Atlassian MCP or community server) |
