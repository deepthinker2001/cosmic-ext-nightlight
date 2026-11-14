# Work on issue FV-0M34B1: Audit: what the reviews of Engineer 1's work on…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-0m34b1-audit-what-the-reviews-of-engineer-1-s-w` · FV-0M34B1: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Docs)

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
| `.github/workflows/release.yml` | modified | +9 −9 |  |

## Tested

- `cargo test`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Moved signature upload after apt publication; cargo tests pass, formatting is clean, and clippy reports no warnings. Checking workflow order and README shell behavior next.

## Summary

The README already uses a quoted `$deb` variable throughout verification and installation, so that finding is resolved. I’ll preserve it and move the signature upload after repository publication.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `bde86f2cf3` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
