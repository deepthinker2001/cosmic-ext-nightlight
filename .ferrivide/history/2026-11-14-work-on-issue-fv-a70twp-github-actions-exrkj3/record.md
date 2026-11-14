# Work on issue FV-A70TWP: GitHub Actions CI

**Merged** 2026-11-14 · Engineer 2 (project agent) · `claude-sonnet-5-5` · branch `fv-a70twp-github-actions-ci` · FV-A70TWP: GitHub Actions CI

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-A70TWP: GitHub Actions CI
>
> Add a workflow run on push and pull request: cargo fmt --check, clippy, tests and a locked build, using the libcosmic system dependencies. Match the user's other projects' workflows where they are available.
>
> Labels: Release

Then:

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The workflow does what was asked. fmt --check, clippy -D warnings, test --locked and the build all pass locally, and the Rust changes are formatting only. Because it's a pull_request trigger and not pull_request_target, fork PRs don't run with secrets. Two supply-chain hardening gaps remain in the workflow: the GITHUB_TOKEN's scope isn't limited, and third-party actions are pinned to mutable tags.
> #1 [must fix] .github/workflows/ci.yml:7: There's no `permissions:` block, so the job gets the repository's default GITHUB_TOKEN scope, which is read-write on older repos and orgs. This CI only needs to read the code. actions/checkout also keeps that token in .git/config by default (persist-credentials: true), where every later step can read it, including build scripts and proc-macros from the whole dependency tree that run during clippy, test and build. A compromised crate or action could then push to the repo. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Add at the top level: permissions: contents: read and on checkout: - uses: actions/checkout@<sha> # v4 with: persist-credentials: false
> #2 [must fix] .github/workflows/ci.yml:25: The third-party actions `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2` (line 29), and `actions/checkout@v4` (line 14), are pinned to tags or branches their owners can move. Whatever code they point to runs with the job's token and source. `@stable` is a branch, so it changes by design. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Pin each action to a full commit SHA with the tag in a comment, e.g. `uses: dtolnay/rust-toolchain@<40-char sha> # stable` with `toolchain: stable` passed as input, and `Swatinem/rust-cache@<sha> # v2.x.y`. Optionally add Dependabot (`package-ecosystem: github-actions`) to keep the pins current.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> [Event] Your other reviewers' findings, to fix along with the changes asked for:
> #3 [suggestion] .github/workflows/ci.yml:1: The workflow has no `permissions:` block, so the GITHUB_TOKEN gets the repository's default scope. A CI job that only reads code should get read-only access. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Add `permissions:\n  contents: read` at the top level, after `on:`.
> #4 [suggestion] .github/workflows/ci.yml:14: `just` is installed but no step uses it. Either drop it from the apt list or leave a comment saying it's kept to match the project's justfile-based workflow. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Remove `just` from the apt-get line, or keep it and note why.
> #5 [suggestion] .github/workflows/ci.yml:5: Add a concurrency group with cancel-in-progress, and `permissions: contents: read`, to avoid duplicate runs and limit the token's scope. Optional. (Sonnet 5.5)
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/dependabot.yml` | added | +6 −0 |  |
| `.github/workflows/ci.yml` | added | +51 −0 |  |
| `src/gamma.rs` | modified | +34 −8 |  |
| `src/main.rs` | modified | +16 −3 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): All five findings from round 1 are addressed in caea889. The workflow now has a top-level `permissions: contents: read`, checkout uses `persist-credentials: false`, and all three actions are pinned to full commit SHAs with the tag in a comment. Unused `just` is removed, and a concurrency group with cancel-in-progress is added. A Dependabot config for github-actions is added too. The only changed lines are in ci.yml and dependabot.yml. I didn't check the pinned SHAs against upstream, since this light round doesn't re-run the build and I had no network check for them. Nothing blocks the merge.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): All five earlier findings are addressed: read-only permissions, persist-credentials false, actions pinned to commit SHAs with the tag in a comment, `just` removed, a concurrency group added, and Dependabot added for github-actions. I couldn't check the pinned SHAs against upstream, so confirm they match the stated tags (the checkout comment says v4.4.0).

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): Both must-fix findings are fixed. The token is now read-only (`permissions: contents: read`), and checkout runs with `persist-credentials: false`. All three actions are pinned to full commit SHAs, and I checked each one with git ls-remote against its upstream: checkout v4.4.0, the commit rust-cache's v2.9.3 tag points to, and the current head of rust-toolchain's stable branch. The new concurrency group and the Dependabot config for github-actions add no risk, and no Rust source changed since the last round.

## Summary

I fixed all five findings in `.github/workflows/ci.yml` and committed. I haven't run the workflow on GitHub, and I didn't validate the YAML locally.

- **#1 and #3, token scope:** the workflow now has a top-level `permissions: contents: read`. The checkout step sets `persist-credentials: false`, so later build steps can't read the token.
- **#2, pinned actions:** all three actions are pinned to full commit SHAs, each with its tag in a comment. I got the SHAs from the actions' GitHub repositories:
  - `actions/checkout` is pinned to v4.4.0.
  - `Swatinem/rust-cache` is pinned to v2.9.3.
  - `dtolnay/rust-toolchain` is pinned to the current head of its `stable` branch. I pass `toolchain: stable` explicitly.
- **#2, Dependabot:** I added `.github/dependabot.yml` with a weekly `github-actions` update, which keeps the pins current.
- **#4:** I removed `just` from the apt list because no step uses it.
- **#5:** I added a concurrency group with `cancel-in-progress: true`.

None of the findings are declined.

---

From `806260f1f1` to `caea889786`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
