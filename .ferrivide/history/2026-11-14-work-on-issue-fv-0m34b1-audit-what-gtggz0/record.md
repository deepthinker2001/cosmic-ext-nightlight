# Work on issue FV-0M34B1: Audit: what the reviews of Engineer 1's work on…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-0m34b1-audit-what-the-reviews-of-engineer-1-s-w` · FV-0M34B1: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Docs)

0 tokens in, 0 out

## Asked

> Work on issue FV-0M34B1: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Docs)
>
> Findings that couldn't be resolved in that change (R24), each with why:
>
> - [suggestion] .github/workflows/release.yml:241: The .asc is uploaded in the apt job before the 'Publish the repository' step. If the push fails, the release page has a signature but the repository isn't updated. This is harmless, because the key is the same. You could move the upload step after the publish step so the two stay in step.
>   - Why it's left for later: a suggestion raised in a re-review round, which can't hold the merge
>   - Raised by Sonnet 5.5 on Engineer 1's work on cosmic-ext-nightlight
> - [suggestion] README.md:151: The globs `cosmic-ext-nightlight_*.deb.asc cosmic-ext-nightlight_*.deb` expand to several arguments when more than one version is in the folder. gpgv then treats the extra files as data and reports a confusing BAD signature (it fails safe), and `apt install ./…*.deb` would install several files.
>   - Why it's left for later: a suggestion raised in a re-review round, which can't hold the merge
>   - Suggested fix: Tell users to run it in a folder holding just the one download, or set `deb=cosmic-ext-nightlight_X.Y.Z_amd64.deb` and use "$deb" / "$deb.asc" / "$deb.sha256".
>   - Raised by Cyber reviewer (Opus 5.5) on Engineer 1's work on cosmic-ext-nightlight
>
> Labels: audit, Docs
>
> Fix both findings: move the .asc upload in release.yml's apt job after the 'Publish the repository' step; in README.md (~line 151) make the verify/install commands use a single $deb variable (deb=cosmic-ext-nightlight_X.Y.Z_amd64.deb, "$deb", "$deb.asc", "$deb.sha256") instead of globs. Don't write the fingerprint placeholder token anywhere new. Commit messages must describe the work, never mention engineers.

## Understood

Resolve both audit findings by publishing the apt repository before uploading the detached release signature, and ensuring README verification/install commands select one .deb via a quoted deb variable.

Assumptions:

- The README finding is already resolved in the current checkout and should be preserved, not rewritten unnecessarily.
- Moving the upload step without an always() condition will keep it gated on successful repository publication.

Plan:

- Move only the signature-upload step after Publish the repository.
- Validate workflow step order and exercise the README command chain with multiple package filenames and failure cases.
- Run required formatting, clippy and tests; commit the focused change.

Out of scope:

- Changing signing keys or fingerprint text.
- Running a live GitHub release or pushing the apt repository.
- Changing application behavior.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +9 −9 | Reordered apt job steps so detached signature attachment follows successful repository publication. |

### Decisions

- **Move the unchanged signature-upload step after Publish the repository.**: GitHub Actions' default success gating prevents attachment after a failed publication. Turned down: Keep attachment before publication..
- **Leave README unchanged because it already uses the requested quoted deb variable.**: Current instructions already select one download and chain verification and installation with &&; mocked shell checks confirmed behavior with multiple downloads and failed verification. Turned down: Rewrite already-correct instructions unnecessarily..

## Tested

- `cargo test`: 47 passed, 0 failed, 1 skipped
- `cargo test`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Moved signature upload after apt publication; cargo tests pass, formatting is clean, and clippy reports no warnings. Checking workflow order and README shell behavior next.
- Committed 1e0c92f210: signature upload now follows apt publication; confirmed README already selects one package safely. All local checks pass; no live GitHub release was run.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`chatgpt:gpt-6-luna`): The release signature upload now follows repository publication, and the README verification/install instructions already use one explicit `$deb` filename consistently. The release workflow parses as YAML and the changed files have no whitespace errors; no Rust build or tests are relevant to this workflow-only change.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The .asc upload now runs after 'Publish the repository'. The README already used a quoted $deb variable (lines 152-156), so no change was needed there. The unchanged-repository early `exit 0` ends only that step, so the upload still runs. The commit message is clean and adds no placeholder token.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The only change moves the release workflow's .asc upload step, unchanged, to after "Publish the repository". Its token scope, environment and quoting are the same as before. Each step starts in the workspace root, so the `cd site` in the publish step doesn't affect it. The publish step's "unchanged" path ends with `exit 0`, so the upload still runs on a redo. The README finding was already resolved before this change (a quoted single "$deb" and no globs). YAML parses, cargo test passes (47 passed, 1 ignored), and the commit message follows C4.

## Summary

Committed as **`1e0c92f210`** — `Fixes FV-0M34B1`.

- Moved the `.asc` upload after **Publish the repository**, so failed publication prevents signature attachment.
- Confirmed the README already uses one quoted `$deb` variable for verification and installation; no README edit was needed. No fingerprint placeholder was added.

**Validation:** 47 tests passed, 1 ignored; formatting and clippy passed. Workflow order and shell syntax checks passed. Mocked README commands confirmed that multiple downloads don’t affect package selection and failed signature or hash checks block installation.

No live GitHub release was run.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `bde86f2cf3` to `1e0c92f210`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
