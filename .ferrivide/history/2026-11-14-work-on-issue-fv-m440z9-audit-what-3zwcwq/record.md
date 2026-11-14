# Work on issue FV-M440Z9: Audit: what the reviews of Engineer 1's work on…

**Merged** 2026-11-14 · Engineer 2 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-m440z9-audit-what-the-reviews-of-engineer-1-s-w` · FV-M440Z9: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Docs)

0 tokens in, 0 out

## Asked

> Work on issue FV-M440Z9: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Docs)
>
> Findings that couldn't be resolved in that change (R24), each with why:
>
> - [must_fix] .github/workflows/release.yml:20: The trigger is `push: branches: [main]`, not `push: tags: ['v*']`. The task required that pushing a `v*` tag starts the release, and that the release:published trigger is dropped. Tag pushes now start nothing, so the issue's goal (`git tag vX.Y.Z && git push origin vX.Y.Z` is enough) is not met.
>   - Why it's left for later: it needs a decision from the user: The reviewer checked against the original issue (tag-push trigger). The project manager then replaced it, at the user's request, with: push to main releases when v<version> has no tag, no tag push needed. Going back to the tag trigger needs the user to say so.
>   - Suggested fix: Restore `on: push: tags: ['v*']` alongside `workflow_dispatch`. Set TAG from `github.ref_name` on tag pushes (or `inputs.tag` on dispatch). Keep the run on the tagged commit. If the user prefers the untagged-version-on-main design, get that agreed first and say so in the task, rather than changing the design silently.
>   - Raised by Quick reviewer (Haiku 5.5) on Engineer 1's work on cosmic-ext-nightlight
> - [should_fix] docs/RELEASING.md:130: The docs now say to push `main` to release, and that the `release` environment must allow branch `main`. The task said the environment already allows `v*` tags and that the docs should say to push a tag. The environment rule change is not something the task asked for, and it is a manual GitHub step the user has to make.
>   - Why it's left for later: it needs a decision from the user: Docs follow the main-push design. The release environment must allow branch main for apt to run from a main push; the manager asked for this to be stated in docs/RELEASING.md, and it is.
>   - Suggested fix: Rewrite the Cut a release section to say: `git tag vX.Y.Z && git push origin vX.Y.Z` (the environment already allows `v*` tags), and drop the branch-main requirement unless the trigger is changed to main on purpose.
>   - Raised by Quick reviewer (Haiku 5.5) on Engineer 1's work on cosmic-ext-nightlight
>
> Labels: audit, Docs

Then:

> [Message from Project manager] Agreed. Keep the push-to-main release design (the user's explicit choice). Add only the short clarification in the release guide that pushing a tag does not start a release. Don't change the workflow or the release environment. Commit messages must describe the change and not name agents.

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [suggestion] .github/workflows/release.yml:183: This comment says the `release` environment is "limited to v* tags and main". The docs, including the sentence this change adds, say branch `main` only, with no Tag rule. A maintainer who goes by the workflow comment could add a `v*` tag rule to the environment, which widens who can reach the signing key. The comment is older than this change, but this change states the stricter rule, so the two now disagree openly. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Change the comment to: `# limited to branch main, no tag rule), not in repository secrets.`
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Understood

Audit the two deferred release-trigger/documentation findings in FV-M440Z9 against the current implementation and recorded design decision.

Assumptions:

- The explicit user-requested main-push design recorded in FV-6BE3MK remains authoritative; the audit is not a new instruction to reverse it.
- A reader-facing clarification resolves the ambiguity without changing release behavior or GitHub environment settings.

Plan:

- Preserve the main-push workflow and main-only signing environment.
- Clarify in the release guide that main-push intentionally replaces tag-push instructions and that manually pushing a tag does not start a release.
- Run formatting, clippy and tests per C2, inspect the documentation diff, and commit the focused clarification.

Out of scope:

- Restoring tag-push releases contrary to the recorded user decision.
- Publishing a release or modifying live GitHub settings.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +1 −1 | Updated the signing-environment comment to branch main only, no tag rule, resolving cyber finding #1. |
| `docs/RELEASING.md` | modified | +4 −1 | Earlier commit in this change clarified that tag pushes do not start releases. |

### Decisions

- **Preserved main-push releases and the main-only signing environment.**: Engineering history records the user's explicit choice, and the project manager confirmed it during this audit; both findings compare against superseded tag-push instructions. Turned down: Restore tag-push releases: rejected because it would reverse the user's chosen design..
- **Added an explicit warning where maintainers cut releases.**: Documentation review identified that readers following older tag-push instructions need to know that pushing a tag does not trigger anything, not merely that a tag is unnecessary. Turned down: Leave documentation unchanged: rejected because the historical instructions remain easy to confuse with the current procedure..
- **Corrected the stale workflow comment rather than changing the environment policy.**: The approved design limits signing to main with no tag rule; documentation must not invite maintainers to widen key access. Turned down: Allow v* tags in the signing environment: rejected as contrary to the chosen policy..

## Tested

- `cargo test --locked`: 47 passed, 0 failed, 1 skipped
- `cargo test --locked`: 47 passed, 0 failed, 1 skipped
- `cargo test --locked`: 47 passed, 0 failed, 1 skipped
- `cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Release-guide clarification added; fmt and clippy pass, tests: 47 passed, 0 failed, 1 ignored.
- Committed 7739e6b21c: clarified intentional main-push releases; workflow and environment unchanged; all C2 checks pass.
- Fixed cyber finding #1: workflow comment now states main-only environment with no tag rule; fmt, clippy and tests pass.
- Committed 9a7b108e36 resolving cyber finding #1; signing-environment comment matches the guide, with no behavior changes.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`chatgpt:gpt-6-luna`): The previously raised workflow-comment inconsistency is fixed: it now states the release environment is limited to branch main with no tag rule. The release guide clarification remains unchanged, and the validation recorded for this comment-only change passed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The workflow comment in release.yml now says the environment is limited to branch main with no tag rule, matching the docs. The last round's finding is fixed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): Finding #1 is fixed. The comment above the `apt` job's environment in release.yml now says "limited to branch main, no tag rule", which matches docs/RELEASING.md. The only change since the last round is that one comment line, and the workflow still parses. Nothing new to raise.

## Summary

Fixed review finding #1 in **9a7b108e36**.

The workflow comment now says the signing environment is limited to **branch `main`, with no tag rule**, matching the release guide. No workflow behavior or live GitHub settings changed.

Checks passed: formatting, clippy, whitespace check, and tests (**47 passed, 0 failed, 1 ignored**).

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `1fa92a879b` to `9a7b108e36`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
