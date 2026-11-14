# Work on issue FV-0A2C5R: Prepare release 0.3.0: version bump and changelog

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-0a2c5r-prepare-release-0-3-0-version-bump-and-c` · FV-0A2C5R: Prepare release 0.3.0: version bump and changelog

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-0A2C5R: Prepare release 0.3.0: version bump and changelog
>
> Prepare the next release; do not push, tag or release (the user pushes).
> - Bump to 0.3.0 (minor: the manual schedule was removed and the location toggle added) in Cargo.toml and Cargo.lock.
> - Turn CHANGELOG.md's Unreleased entries into a 0.3.0 section dated today (use the clock tool), covering: the performance and click fixes, X-HostWaylandDisplay in the desktop file, saves moved off the UI thread, manual schedule removed, "From time zone" toggle, apt arch=amd64 line, apt landing page, Ubuntu 24.04 build host. Keep it short and plain.
> - Check README and docs/RELEASING.md for anything stale about the schedule or version.
> - Run fmt, clippy and the tests; also run `cargo deb --locked` and scripts/check-deb-libc.py on the result if the tools exist. Say in your report what to do next for the user: merge, then `git push`.
>
> Labels: Docs

## Understood

Prepare release 0.3.0: bump version in Cargo.toml/Cargo.lock, turn CHANGELOG Unreleased into a dated 0.3.0 section covering the listed changes, fix stale README/RELEASING text, run fmt/clippy/tests and cargo deb + libc check if available. No push/tag.

Assumptions:

- 0.2.1 (with the 24.04 build host and the landing-page script) is already on origin/main; the 24.04 host is mentioned in 0.3.0 only as still holding, and the landing page, never changelogged, goes in 0.3.0
- Date is 2026-11-14 per the clock tool

Plan:

- Bump version to 0.3.0 and update Cargo.lock via cargo
- Rewrite CHANGELOG: empty Unreleased, 0.3.0 - 2026-11-14 section with Changed/Fixed/Packaging
- Fix RELEASING.md's stale 'build uses ubuntu-latest' sentence
- Run fmt, clippy, tests, cargo deb --locked, check-deb-libc.py
- Commit

Out of scope:

- Pushing, tagging, releasing
- Rewriting RELEASING's broader 'Not tested' section beyond the stale runner claim

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `CHANGELOG.md` | modified | +21 −7 |  |
| `Cargo.lock` | modified | +1 −1 |  |
| `Cargo.toml` | modified | +1 −1 |  |
| `docs/RELEASING.md` | modified | +3 −2 |  |

## Tested

- `cargo test --locked`: 56 passed, 0 failed, 1 skipped

Progress reports:

- 0.3.0 bump and changelog done; fmt, clippy clean, 56 tests pass (1 ignored). Now trying cargo deb.

## Summary

The package builds as 0.3.0-1, and the libc check passes (`libc6 (>= 2.39)`). Committing.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `d80c65f9be` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
