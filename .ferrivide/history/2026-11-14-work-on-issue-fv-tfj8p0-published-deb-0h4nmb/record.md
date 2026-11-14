# Work on issue FV-TFJ8P0: Published .deb needs libc6 >= 2.43, so it won't…

**In progress** 2026-11-14 · Engineer 2 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-tfj8p0-published-deb-needs-libc6-2-43-so-it-won` · FV-TFJ8P0: Published .deb needs libc6 >= 2.43, so it won't install on Ubuntu 24.04

0 tokens in, 0 out

## Asked

> Work on issue FV-TFJ8P0: Published .deb needs libc6 >= 2.43, so it won't install on Ubuntu 24.04
>
> Installing from the apt repo on Ubuntu 24.04 (glibc 2.39) fails: "cosmic-ext-nightlight : Depends: libc6 (>= 2.43) but 2.39-0ubuntu8.9 is to be installed". cargo-deb takes the libc6 floor from the build host, and `ubuntu-latest` is now a newer release (see knowledge k3: build on the oldest target distro).
>
> Do:
> - In .github/workflows/release.yml, run the `build` job (the one that runs cargo deb) on `ubuntu-24.04` instead of `ubuntu-latest`, with a comment saying why. Check CI (ci.yml) too: build there on ubuntu-24.04 so a build-host problem shows before release. Keep the other jobs' behaviour unchanged.
> - Check that libcosmic's build dependencies (apt packages installed in the workflow) exist on 24.04 and fix the package list if not.
> - Add a check step after `cargo deb` that fails the job if the .deb's libc6 floor (dpkg-deb -f ... Depends) is above 2.39, so this can't recur silently.
> - Note in docs/RELEASING.md that releases are built on the oldest supported distro (Ubuntu 24.04) and why.
> - Say in your report that an already-published 0.2.0 .deb has the wrong dependency, so a new version (bump in Cargo.toml and Cargo.lock, CHANGELOG) is needed to publish a fixed one; do not release, the user does that. Do the bump in this change.
>
> Labels: Docs

Then:

> [Message from Project manager] Your edits are in place and fmt/clippy/tests were run. Don't redo anything: commit now, then report_status with the summary, files and test results, and stop.

> Why aren't you working?

> [Message from Project manager] Please commit your FV-TFJ8P0 work now (ubuntu-24.04 build, libc6 guard, 0.2.1 bump), re-run fmt/clippy/tests if anything changed, then report_status and finish your turn so reviews can start.
>
> [Issue FV-TFJ8P0, as it stands now]
> FV-TFJ8P0: Published .deb needs libc6 >= 2.43, so it won't install on Ubuntu 24.04
> Status: In Progress
> Labels: Docs
> Origin: made in Ferrivide by Project manager
>
> Installing from the apt repo on Ubuntu 24.04 (glibc 2.39) fails: "cosmic-ext-nightlight : Depends: libc6 (>= 2.43) but 2.39-0ubuntu8.9 is to be installed". cargo-deb takes the libc6 floor from the build host, and `ubuntu-latest` is now a newer release (see knowledge k3: build on the oldest target distro).
>
> Do:
> - In .github/workflows/release.yml, run the `build` job (the one that runs cargo deb) on `ubuntu-24.04` instead of `ubuntu-latest`, with a comment saying why. Check CI (ci.yml) too: build there on ubuntu-24.04 so a build-host problem shows before release. Keep the other jobs' behaviour unchanged.
> - Check that libcosmic's build dependencies (apt packages installed in the workflow) exist on 24.04 and fix the package list if not.
> - Add a check step after `cargo deb` that fails the job if the .deb's libc6 floor (dpkg-deb -f ... Depends) is above 2.39, so this can't recur silently.
> - Note in docs/RELEASING.md that releases are built on the oldest supported distro (Ubuntu 24.04) and why.
> - Say in your report that an already-published 0.2.0 .deb has the wrong dependency, so a new version (bump in Cargo.toml and Cargo.lock, CHANGELOG) is needed to publish a fixed one; do not release, the user does that. Do the bump in this change.

## Understood

Fix Ubuntu 24.04 installation compatibility by building CI and release packages on ubuntu-24.04, rejecting .deb libc6 requirements above 2.39, documenting the oldest-supported-distro policy and preparing version 0.2.1.

Assumptions:

- 0.2.1 is the next patch version after published 0.2.0.
- Existing apt dependencies can remain: every package has an Ubuntu noble main/universe archive entry.
- Only build runners change; attach/apt jobs and existing security separation remain unchanged.

Plan:

- Pin the CI and release build runners with explanatory comments.
- Add a fail-closed libc6 guard after cargo deb and exercise it with real synthetic .deb packages.
- Bump manifest/lockfile and changelog, document compatibility and verify formatting, clippy, tests and packaging where available.
- Commit the verified change without pushing or releasing.

Out of scope:

- Publishing or replacing release artifacts.
- Changing signing, tagging or apt publication behavior.
- Application Rust behavior changes.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/ci.yml` | modified | +6 −1 |  |
| `.github/workflows/release.yml` | modified | +7 −1 |  |
| `CHANGELOG.md` | modified | +10 −0 |  |
| `Cargo.lock` | modified | +1 −1 |  |
| `Cargo.toml` | modified | +1 −1 |  |
| `docs/RELEASING.md` | modified | +14 −0 |  |
| `scripts/check-deb-libc.py` | added | +36 −0 |  |
| `scripts/test-check-deb-libc.py` | added | +64 −0 |  |

## Tested

No test runs were reported.

## Summary

The real package built on 24.04 asks for `libc6 (>= 2.39)` and the guard accepts it, which confirms the fix end to end. Committing:

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `6db778d492` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
