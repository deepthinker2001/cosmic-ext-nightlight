# Work on issue FV-CK2Y04: Audit: what the reviews of Engineer 1's work on…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-ck2y04-audit-what-the-reviews-of-engineer-1-s-w` · FV-CK2Y04: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Release)

0 tokens in, 0 out

## Asked

> Work on issue FV-CK2Y04: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later (Release)
>
> Findings that couldn't be resolved in that change (R24), each with why:
>
> - [should_fix] README.md:73: The fingerprint the README tells users to compare against is still the placeholder `FINGERPRINT-TO-BE-FILLED-IN-BY-THE-MAINTAINER-BEFORE-THE-FIRST-RELEASE`. Round-1 finding #5 asked for the real fingerprint, so the verify steps can't be checked yet. This must be filled in before the first tag is pushed.
>   - Why it's left for later: it needs a decision from the user: The real 40-character fingerprint depends on the signing key, which the maintainer has to create or choose; I don't have it. The README keeps a placeholder, and the build job now fails any release while the placeholder is still there, so no release can go out without it.
>   - Suggested fix: Once the signing key exists, put its full 40-character fingerprint in that code block. Until then, keep the placeholder, and check the README is filled in before the first release.
>   - Raised by Quick reviewer (Haiku 5.5) on Engineer 1's work on cosmic-ext-nightlight
>
> Labels: audit, Release

## Understood

Audit FV-CK2Y04's deferred signing-key fingerprint finding and ensure the release instructions use the maintainer-selected fingerprint.

Assumptions:

- The fingerprint supplied by the user in merged session 2026-11-14-in-readme-md-replace-the-signing-key-63pbc1 is authoritative; no new key should be generated.
- README already resolves the placeholder finding, so avoid changing its verified trust steps.

Plan:

- Confirm both README verification paths contain the user-supplied full fingerprint.
- Add an explicit maintainer pre-release check that the selected key and environment fingerprint match the README.
- Run documentation consistency checks and the contract's Rust checks/tests, then commit the focused documentation change.

Out of scope:

- Accessing signing secrets or creating/rotating a signing key.
- Publishing a release or claiming the remote key or GitHub environment has been verified.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `docs/RELEASING.md` | modified | +9 −1 | Require the trusted signing key's primary fingerprint and release environment variable to match the published README fingerprint before releasing; stop on mismatch. |

### Decisions

- **Retain the existing README fingerprint and verification flow.**: A merged session records the user's explicit fingerprint; both current README occurrences match it, so the deferred placeholder finding is stale. Turned down: Generate a new signing key or ask the user for the already-recorded fingerprint..
- **Add the fingerprint comparison before merging a version bump.**: The current workflow releases from main automatically; a check before pushing a tag would be too late. This addresses the release-specific timing of the trust check. Turned down: Only report that the placeholder was already removed.; Change the fingerprint based on a downloaded key..

## Tested

- `cargo test --locked`: 47 passed, 0 failed, 1 skipped
- `cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Fingerprint audit checks pass; cargo test: 47 passed, 0 failed, 1 ignored; fmt and clippy clean. Preparing the focused release-checklist commit.
- FV-CK2Y04 audited and committed as a7895b8fc3: README already has the user-provided fingerprint; release checklist now requires checking trusted key and environment variable before release. Live key/environment not accessed.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`chatgpt:gpt-6-luna`): The added maintainer release checklist addresses the unresolved fingerprint audit item and uses the same fingerprint already published in the README. It is limited to the requested documentation change; formatting, clippy, and tests pass.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Docs-only change to docs/RELEASING.md. The README already holds the real fingerprint, and the one added in RELEASING.md matches it (spaces removed). The anchor #with-apt-recommended exists and APT_SIGNING_KEY_ID is the variable the workflow reads. No code changed, so I didn't run the tests or build, and the smell scan found nothing.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change only touches docs: it adds a check to step 1 of docs/RELEASING.md that the signing key's full fingerprint and the `release` environment's APT_SIGNING_KEY_ID both match the published fingerprint. That fingerprint matches README.md lines 107 and 145, the #with-apt-recommended anchor resolves, and no placeholder is left anywhere in the tree. It also tells the maintainer not to replace the documented fingerprint to match a downloaded key, which is the right way round. The change has no secrets, no code and no workflow edits, and the commit message keeps C4. cargo test --locked: 47 passed, 1 ignored.

## Summary

Committed as `a7895b8fc3` — **Document signing-key fingerprint check before release**.

The README placeholder was already resolved in a merged change. Both verification paths contain the maintainer-provided fingerprint:
`0472CEF01CE0E79EC118662D34A06EB979E58584`.

I updated `docs/RELEASING.md` to require checking the trusted signing key and `APT_SIGNING_KEY_ID` against that fingerprint **before merging the version change**, since releases now start automatically from `main`.

Validation:
- Tests: **47 passed, 0 failed, 1 ignored**
- Formatting and Clippy: clean
- Documentation consistency and diff checks: passed

I did not access the live signing key or GitHub environment, and did not publish a release.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `376bbf9531` to `a7895b8fc3`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
