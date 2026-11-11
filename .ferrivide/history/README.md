# Engineering history

Each folder here is one AI agent session that worked on this project, kept
by Ferrivide (the IDE) with each of the agent's commits and when its work
was merged or discarded, or imported from Claude Code's session logs. Together they explain how the project came to be as it is: what
was asked, what the agent understood, what it changed and why, how it was
tested and reviewed, and how it summed up. They are ordinary files, so they
last as long as the code does, even when commit history is squashed or
pruned and branches are deleted.

Folders are named `<date>-<subject>-<agent>`; sorted by name, they're in
the order the sessions started.

## A session's files

- `record.md`: the account, for people.
- `record.json`: the same, as data (see below).
- `changes.diff`: the work's whole diff (`git diff <base_commit>..<head_commit>`).
  Starting from today's files, undoing these diffs newest first walks back
  through the project's earlier states, with or without git.
- `transcript.jsonl`: the conversation, one step a line (see below),
  shortened to what matters.
- `review-N.jsonl`: each reviewer agent's conversation, in the same form.

## record.json

| Field | Meaning |
|---|---|
| `format` | The version of these files' format (1). |
| `id`, `agent` | The agent: its id, name, kind (project, workspace or all-workspaces agent), model and effort. |
| `title` | What the session was about (its first request's first line). |
| `outcome` | `merged`; `discarded` (its work was thrown away: what was tried and why it was dropped); `in_progress` (kept with each of the agent's commits: the work is under way, or was merged without Ferrivide); or `imported` (brought in from a Claude Code session log, whose work was committed as it went). |
| `project`, `branch`, `base_commit`, `head_commit`, `commits` | Where the work happened: the branch, the commit it started from, its last commit, and all its commits (newest first). |
| `issue` | The issue it was assigned, if any. |
| `started`, `finished` | Unix seconds. |
| `usage` | Tokens used and the estimated cost in US dollars. |
| `requests` | What the user asked, word for word, in order. |
| `understanding` | What the agent understood each request to be: restated, with its assumptions, plan and what it left out of scope. |
| `files` | Each file changed: `path`, `status`, lines `added` and `removed`, and `why` in the agent's words. |
| `decisions` | Choices the agent made: `decision`, `why`, and the `alternatives` it turned down. |
| `tests` | Test runs it reported: `command`, `passed`, `failed`, `skipped`, `failing` tests. |
| `progress` | Its progress reports, in order. |
| `reviews` | Reviewer agents' verdicts (`approve` or `request_changes`), summaries and findings (`file`, `line`, `severity`, `comment`). |
| `contract` | The contract clauses (standards agreed with the user) the work was bound by. |
| `summary` | The agent's closing recap. |

## transcript.jsonl

One JSON object a line, in order. `kind` says which:

| `kind` | Fields | Meaning |
|---|---|---|
| `user` | `text`, `attachments` | What the user (or Ferrivide on their behalf) said; notes of attached files and images. |
| `thinking` | `text` | The agent's reasoning, as the model summarized it. |
| `assistant` | `text` | What the agent said. |
| `tool_call` | `id`, `tool`, `input` | A tool the agent used, with its whole input. `edit_file` (`path`, `old_text`, `new_text`) and `write_file` (`path`, `content`) are its edits. |
| `tool_result` | `id`, `tool`, `output`, `error`, `omitted` | What the tool returned. A file read or directory listing is reduced to its size; command output keeps its start and end. `omitted` counts the bytes left out. |

Secrets (API keys, tokens, private keys, passwords in URLs and the like)
were replaced, before these files were written, with a marker saying what
kind each was, such as `[secret removed: GitHub token]` (Ferrivide's secret
scanner, `ferrivide-scan`; SECURITY.md says what it catches). Older sessions
may say only `[secret removed]`.

## Using it

- To learn why something is the way it is, search the `record.md` files, or
  find the sessions whose `record.json` lists the file under `files`.
- A merge commit names its session in a `History:` line, so `git blame`
  leads from a line to its commit, and from there to the session.
- An agent catching up on the project can read the latest `record.md`
  files, then the transcripts where it needs the detail.
