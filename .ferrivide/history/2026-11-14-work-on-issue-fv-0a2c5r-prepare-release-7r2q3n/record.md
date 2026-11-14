# Work on issue FV-0A2C5R: Prepare release 0.3.0: version bump and changelog

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-0a2c5r-prepare-release-0-3-0-version-bump-and-c` · FV-0A2C5R: Prepare release 0.3.0: version bump and changelog

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
| `CHANGELOG.md` | modified | +21 −7 | Unreleased -> 0.3.0 - 2026-11-14 in Changed/Fixed/Packaging, plus an empty Unreleased |
| `Cargo.lock` | modified | +1 −1 | package version 0.3.0 so --locked builds pass |
| `Cargo.toml` | modified | +1 −1 | version 0.2.1 -> 0.3.0 |
| `docs/RELEASING.md` | modified | +3 −2 | the stale claim that build runs on ubuntu-latest (it's ubuntu-24.04) |

### Decisions

- **Version 0.3.0 (minor)**: A user-visible setting was removed and a new one added; for a 0.x crate that is a minor bump.
- **The 24.04 build host appears in 0.3.0 only as 'still built on 24.04, as since 0.2.1'; the landing page is listed as new in 0.3.0**: Both were merged before 0.2.1 was pushed. The 24.04 change is already in the 0.2.1 notes, but the landing page was never in the changelog. Turned down: List both as new in 0.3.0 (would repeat the 0.2.1 entry); Edit the released 0.2.1 section.

## Tested

- `cargo test --locked`: 56 passed, 0 failed, 1 skipped
- `cargo test --locked`: 56 passed, 0 failed, 1 skipped

Progress reports:

- 0.3.0 bump and changelog done; fmt, clippy clean, 56 tests pass (1 ignored). Now trying cargo deb.
- Release 0.3.0 prepared and committed (7e61f7f7aa): version bump, dated changelog, RELEASING fix; fmt/clippy clean, 56 tests pass, .deb builds with libc6 >= 2.39. Next: merge, then git push main.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The version is 0.3.0 in Cargo.toml and Cargo.lock, and the lockfile is consistent (`cargo metadata --locked` passes). The changelog has a dated 0.3.0 section covering the requested items, and RELEASING.md's stale `ubuntu-latest` mention is fixed. The commit message follows C4. I didn't run fmt, clippy or the tests, since the change touches no code.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Version bump in Cargo.toml and Cargo.lock, a dated 0.3.0 changelog section, and the RELEASING.md runner fix are all correct. fmt is clean and 56 tests pass. README and docs have nothing stale about the schedule. I did not run cargo deb or the libc check; the engineer reported that libc6 (>= 2.39) passes.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change only touches the version (Cargo.toml and Cargo.lock, 0.2.1 → 0.3.0), the changelog and one paragraph of docs/RELEASING.md. It adds no code, dependencies, secrets, scripts or permission changes. The RELEASING.md note matches the workflows: build runs on ubuntu-24.04, while attach and apt run on ubuntu-latest. `cargo test --locked` passes (56 passed, 1 ignored). The commit message names no agents (C4).

## Summary

Release 0.3.0 is ready and committed on this branch as `7e61f7f7aa` (Fixes FV-0A2C5R). Nothing has been pushed, tagged or released.

**What changed:**
- **Version:** `Cargo.toml` and `Cargo.lock` now say 0.3.0.
- **CHANGELOG.md:** the Unreleased notes are now a `0.3.0 - 2026-11-14` section, with an empty Unreleased heading above it:
  - **Changed:** the manual schedule is removed and the "From time zone" switch is added.
  - **Fixed:** `X-HostWaylandDisplay` in the desktop file, and the speed and click fixes, including saves moved off the UI thread.
  - **Packaging:** the apt `arch=amd64` line, the apt landing page, and the Ubuntu 24.04 build host.
- **docs/RELEASING.md:** one sentence was out of date. It said the build job runs on `ubuntu-latest`; it runs on `ubuntu-24.04`.
- **README:** nothing in it was out of date about the schedule or the version.

**One thing to know:** the landing page and the 24.04 build host were both already included in 0.2.1, which is on `origin/main`. The 24.04 change is already in the 0.2.1 notes, so the 0.3.0 notes only say it is "still built on Ubuntu 24.04, as since 0.2.1". The landing page was never in the changelog, so it is listed in 0.3.0.

**Checks:**
- Formatting and clippy are clean, and all 56 tests pass (1 is skipped as before).
- `cargo deb --locked` builds `cosmic-ext-nightlight_0.3.0-1_amd64.deb`, which needs `libc6 (>= 2.39)`.
- `scripts/check-deb-libc.py` passes on that package.

**What to do next:**
1. Before merging, check that the signing key's fingerprint matches the README, as step 1 of docs/RELEASING.md describes.
2. Merge this branch into `main`, then `git push`. The release workflow will see that `v0.3.0` doesn't exist yet, build the package, and create the tag and release.
3. Approve the `release` environment when GitHub asks, and watch the `build`, `attach` and `apt` jobs pass.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `d80c65f9be` to `7e61f7f7aa`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
