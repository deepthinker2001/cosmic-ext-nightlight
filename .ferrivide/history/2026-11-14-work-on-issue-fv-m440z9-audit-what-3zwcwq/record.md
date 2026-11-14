# Work on issue FV-M440Z9: Audit: what the reviews of Engineer 1's work on…

**In progress** 2026-11-14 · Engineer 2 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-m440z9-audit-what-the-reviews-of-engineer-1-s-w` · FV-M440Z9: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Docs)

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
| `docs/RELEASING.md` | modified | +4 −1 |  |

## Tested

- `cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Release-guide clarification added; fmt and clippy pass, tests: 47 passed, 0 failed, 1 ignored.

## Summary

I’ll check the current workflow, release guide, and recorded decision to see whether these findings still apply.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `1fa92a879b` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
