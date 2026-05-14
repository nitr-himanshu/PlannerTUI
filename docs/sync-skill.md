# Sync Skill — PlannerTUI items.json Updater

## Purpose

This skill instructs Claude to populate `.planner_tui/items.json` using data from
services already connected via MCP. No API tokens or credentials are required —
Claude uses the MCP tools already available in your session.

Run this skill by opening it and saying: **"Follow the sync skill."**

---

## How It Works

1. Claude checks which MCP tools are active in the current session
2. Claude reports which data sources it found and asks you for filter conditions
3. Claude fetches the data using those MCP tools
4. Claude writes the results into `.planner_tui/items.json`

You can re-run the skill at any time to refresh the data.

---

## Instructions for AI

Follow every step in order.

---

### Step 1 — Detect Available MCP Data Sources

Inspect the list of tools currently available to you. Look for MCP tools that match
these categories:

| Category     | Look for tool names containing            | maps to items.json key |
| ------------ | ----------------------------------------- | ---------------------- |
| GitHub PRs   | `github` + (`pull_request` or `pr`)       | `github_prs`           |
| GitHub Issues| `github` + `issue`                        | `github_issues`        |
| JIRA         | `jira` + (`search` or `issue`)            | `jira`                 |
| Linear       | `linear` + (`issue` or `search`)          | `jira` (same schema)   |

Report back to the user exactly which categories are available, and which are not.
If none of the above are found, stop and tell the user which MCP servers to install.

---

### Step 2 — Ask for Filter Conditions

For each detected category, ask the user one question to set filters.
Ask all questions in a single message — do not ask one at a time.

**GitHub PRs** (if available):
> Which repos should I watch for PRs? (e.g. `org/repo`) — and should I limit to PRs assigned to you?

**GitHub Issues** (if available):
> Which repos should I watch for issues? Any label filters? Assigned to you only?

**JIRA / Linear** (if available):
> Paste your filter query. For JIRA use JQL (e.g. `project = PROJ AND assignee = currentUser() AND status != Done`). For Linear use a team name or filter description.

Save the user's answers — you will use them in Step 3.

---

### Step 3 — Fetch Data

Using the MCP tools detected in Step 1 and the filters from Step 2, fetch the data.

#### GitHub PRs

Call the GitHub MCP tool to list pull requests for each repo the user specified.
Apply the assignee filter if requested.

Transform each result to:

```json
{
  "id": "<org/repo>#<number>",
  "link": "<html_url>"
}
```

#### GitHub Issues

Call the GitHub MCP tool to list issues for each repo.
Exclude items that are pull requests (check for a `pull_request` field).
Apply label and assignee filters if specified.

Transform each result to:

```json
{
  "id": "<org/repo>#<number>",
  "link": "<html_url>"
}
```

#### JIRA / Linear

Call the JIRA or Linear MCP tool with the user's filter query.

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

### Step 4 — Read Existing items.json

Read `.planner_tui/items.json`. If missing, use this default:

```json
{
  "tasks": [],
  "jira": [],
  "github_prs": [],
  "github_issues": []
}
```

---

### Step 5 — Merge and Write

Apply these rules:

- **`tasks`** — never touch. Preserve exactly as-is.
- **`jira`** — replace with fresh data from Step 3 (JIRA/Linear). If neither MCP was available, leave unchanged.
- **`github_prs`** — replace with fresh data from Step 3. If GitHub PR MCP was unavailable, leave unchanged.
- **`github_issues`** — replace with fresh data from Step 3. If GitHub Issues MCP was unavailable, leave unchanged.

Write the merged result back to `.planner_tui/items.json` with 2-space indentation.

---

### Step 6 — Report

Tell the user:

- How many items were written per category
- Which categories were skipped (MCP not available) and which MCP to add to enable them
- Remind them they can re-run this skill at any time to refresh

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

| Data Source   | MCP Server to install                          |
| ------------- | ---------------------------------------------- |
| GitHub        | `github` MCP (official GitHub MCP server)      |
| JIRA          | `jira` MCP (Atlassian MCP or community server) |
| Linear        | `linear` MCP                                   |
