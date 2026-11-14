# Work on issue FV-TFJ8P0: Published .deb needs libc6 >= 2.43, so it won't…

**Merged** 2026-11-14 · Engineer 2 (project agent) · `claude-opus-5-5` · branch `fv-tfj8p0-published-deb-needs-libc6-2-43-so-it-won` · FV-TFJ8P0: Published .deb needs libc6 >= 2.43, so it won't install on Ubuntu 24.04

About $0.00, 0 tokens in, 0 out

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
| `.github/workflows/ci.yml` | modified | +6 −1 | Runs on ubuntu-24.04 so a build-host problem shows before a release; also runs the guard's tests. |
| `.github/workflows/release.yml` | modified | +7 −1 | The build job runs on ubuntu-24.04 (with a comment saying why) and checks the libc6 floor after cargo deb; attach and apt are unchanged. |
| `CHANGELOG.md` | modified | +10 −0 | 0.2.1 entry describing the libc6 fix. |
| `Cargo.lock` | modified | +1 −1 | Matches the 0.2.1 bump; only this package's entry changed. |
| `Cargo.toml` | modified | +1 −1 | Version 0.2.1, so a fixed package can be published. |
| `docs/RELEASING.md` | modified | +14 −0 | Explains that releases are built on the oldest supported distro (Ubuntu 24.04), and why. |
| `scripts/check-deb-libc.py` | added | +36 −0 | Fails if the .deb's libc6 floor is above 2.39, or if the libc6 dependency is missing or in an unexpected form. |
| `scripts/test-check-deb-libc.py` | added | +64 −0 | Tests the guard against sample .deb files built with dpkg-deb. |

### Decisions

- **Put the check in a script (scripts/check-deb-libc.py) rather than inline shell in the workflow**: Lets CI test it against real sample packages, and it compares versions with dpkg --compare-versions, which orders Debian versions correctly (2.39.1 > 2.39). Turned down: Inline grep/sort -V in release.yml: can't be tested and is easy to get subtly wrong.
- **Fail when the libc6 dependency is missing, doubled or in an unexpected form, not only when it is too new**: If cargo-deb's output ever changes, a guard that ignored unexpected input would let a bad package through unnoticed, which is the failure this change exists to prevent.
- **Keep the apt build-dependency list unchanged**: Each of the existing packages has a package in Ubuntu noble's main or universe archive (checked with apt-cache on a noble system).
- **Bump to 0.2.1 (patch)**: Fixes packaging only, with no change to the applet; apt caches packages by version, so 0.2.0 can't be replaced.

## Tested

- `cargo test --locked; python3 -I scripts/test-check-deb-libc.py; cargo deb --locked + check-deb-libc.py`: 50 passed, 0 failed, 1 skipped

Progress reports:

- FV-TFJ8P0 done (46b1a60bed): CI and the release build run on ubuntu-24.04, a libc6 guard runs after cargo deb, version is 0.2.1. A real .deb built on 24.04 needs libc6 >= 2.39 and passes the guard. Nothing released.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The change does what the task asked. Release and CI both run on ubuntu-24.04, a libc6 floor check runs after the .deb is built, RELEASING.md is updated, and the version is bumped to 0.2.1 in Cargo.toml and Cargo.lock with a CHANGELOG entry. I ran the guard's Python tests and they pass. I did not run the cargo build or tests, and I did not review the apt package list against 24.04.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Release build and CI now run on ubuntu-24.04, a post-`cargo deb` guard fails the job if the libc6 floor is above 2.39, RELEASING.md and the 0.2.1 bump (Cargo.toml, Cargo.lock, CHANGELOG) are in. The guard's tests pass locally. I did not run the Rust build or clippy, and I did not check the apt package list against 24.04.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change moves the release build job and CI to ubuntu-24.04 and adds a libc6 floor check after cargo deb. The check runs only in the build job, which has no secrets and a read-only token, so no trust boundary or privilege changes. The script fails closed on a missing, duplicate or oddly formed libc6 dependency, on a bad version, and on a missing or ambiguous .deb glob. It passes paths and versions to dpkg as argv lists, with no shell. Python guard tests (3), cargo test (47 passed, 1 ignored), fmt and clippy all pass here. Cargo.lock changes only the package version.

## Summary

I'd stalled because the project manager's 8:00 message said "commit now and stop", and I ended my turn without replying. That message was also wrong: I hadn't run the checks yet, and the docs, version bump and changelog weren't done. The work is now finished and committed as `46b1a60bed`.

**Why it broke:** `cargo-deb` sets the `libc6 (>= …)` dependency from the glibc of the machine that builds the package. `ubuntu-latest` is now newer than 24.04, so the 0.2.0 package asked for `libc6 (>= 2.43)` and apt refused it on 24.04, which has glibc 2.39.

**What changed:**
- **Build machine:** the release `build` job and CI both run on `ubuntu-24.04`, each with a comment saying why. The `attach` and `apt` release jobs are unchanged.
- **Build dependencies:** every apt package the workflows install exists on Ubuntu 24.04, so the list stays as it is.
- **New check:** after `cargo deb`, a new step runs `scripts/check-deb-libc.py`, which fails the release if the package asks for a libc6 newer than 2.39. It also fails if the libc6 dependency is missing or in an unexpected form, so a change in `cargo-deb`'s output can't slip a bad package through. CI tests the check against sample packages.
- **Docs:** `docs/RELEASING.md` now says releases are built on the oldest supported Ubuntu (24.04), and why.
- **Version:** bumped to 0.2.1 in `Cargo.toml` and `Cargo.lock`, with a `CHANGELOG.md` entry.

**Checks:**
- `cargo fmt` and `clippy` are clean.
- `cargo test`: 47 passed, 1 ignored.
- The new check's own tests (3) pass.
- Both workflow files still parse.
- I built the real 0.2.1 package on this 24.04 machine. It asks for `libc6 (>= 2.39)` and passes the new check.
- Nothing has run on GitHub yet.

**For you:** the published 0.2.0 package has the wrong dependency and can't be fixed in place, because apt caches packages by version. Merging this and pushing `main` releases 0.2.1 through the existing workflow, which is the version that installs on 24.04. I haven't released or pushed anything.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `6db778d492` to `46b1a60bed`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
