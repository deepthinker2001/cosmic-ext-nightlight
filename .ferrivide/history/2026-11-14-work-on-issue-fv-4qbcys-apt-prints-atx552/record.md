# Work on issue FV-4QBCYS: apt prints a notice about i386 for our repository; add…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-4qbcys-apt-prints-a-notice-about-i386-for-our-r` · FV-4QBCYS: apt prints a notice about i386 for our repository; add arch=amd64 to the sources line

0 tokens in, 0 out

## Asked

> Work on issue FV-4QBCYS: apt prints a notice about i386 for our repository; add arch=amd64 to the sources line
>
> On a system with i386 enabled, apt update prints: "N: Skipping acquire of configured file 'main/binary-i386/Packages' as repository '... stable InRelease' doesn't support architecture 'i386'". It's harmless, but noisy. The repo is amd64 only.
>
> Do:
> - Add `arch=amd64` to the `deb [...]` sources line, so it reads `deb [arch=amd64 signed-by=/usr/share/keyrings/cosmic-ext-nightlight.gpg] ...`, in README.md and in the index.html heredoc in scripts/publish-apt.sh. Keep the two identical (the publisher's local test compares them; update the test if it needs it). Keep the fingerprint-first steps unchanged.
> - Add a line in the README telling existing users to edit /etc/apt/sources.list.d/cosmic-ext-nightlight.list the same way if they want the notice gone.
> - Add a CHANGELOG line under the unreleased/next entry. Do not bump the version again unless the changelog layout needs it; do not release.
>
> Labels: Docs

## Understood

Add the amd64 architecture filter to both documented apt source lines, explain the opt-in correction for existing installs, and note it under Unreleased without releasing or bumping versions.

Assumptions:

- The fingerprint-first trust flow and publisher security checks must stay unchanged.
- The earlier publisher comparison was a local scratch test, not a checked-in test; repeat that comparison against generated output.
- Project manager owns requirements/features.

Plan:

- Update README and HTML heredoc identically, plus migration guidance and changelog.
- Run a real local publisher with a throwaway key and dummy package, compare install blocks with README, and check unchanged fingerprint instructions.
- Run cargo fmt, clippy and tests per C2, review diff and commit.

Out of scope:

- Live apt configuration or deployment
- Version changes or releases
- Applet code changes

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `CHANGELOG.md` | modified | +4 −0 | Record the notice fix under Unreleased without a version bump. |
| `README.md` | modified | +6 −1 | Add architecture filter and precise optional migration guidance for existing users. |
| `scripts/publish-apt.sh` | modified | +1 −1 | Match README architecture filter in generated index.html. |

### Decisions

- **Add only arch=amd64 to both install commands while leaving fingerprint-first steps intact.**: Avoids irrelevant i386 acquisition without weakening trust or changing repository behavior. Turned down: Changing repository metadata or trust setup.
- **Document an optional existing-user edit rather than telling readers to repeat setup.**: Documentation specialty: changing new-install snippets alone would not remove the notice for current users, and re-importing the key is unnecessary. Turned down: Only update new-install instructions; Repeat the entire setup procedure.

## Tested

- `cargo fmt --all -- --check && cargo clippy --all-targets && cargo test --locked`: 47 passed, 0 failed, 1 skipped
- `cargo fmt --all -- --check && cargo clippy --all-targets && cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Local publisher generated and signed the test repo; all three HTML install blocks match README. C2 checks passed: fmt/clippy clean, 47 Rust tests passed, 1 ignored.
- Committed face134876 for FV-4QBCYS: matching amd64-only sources, existing-user guidance and Unreleased note; publisher integration and C2 checks passed. No release or version bump.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The change matches the task. README and the publish-apt.sh heredoc both get `arch=amd64` on the sources line, and the two lines are identical. The README has a note for existing users and CHANGELOG has an Unreleased entry. There is no version bump, and the fingerprint steps are unchanged. I read the diff but did not run the publisher's local test.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The README line and the publisher heredoc line are identical and both now carry arch=amd64. No other signed-by line lacks it. The README note for existing users and the Unreleased changelog line are in place, and the fingerprint steps are untouched. I did not run the publisher's local test or the Rust checks, because only docs and a shell heredoc changed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): This is a docs-only change. It adds arch=amd64 to the documented apt sources line in README.md and the site page made by publish-apt.sh, plus a README note and a CHANGELOG entry. signed-by= is still on the line, so the keyring pin isn't weakened. The fingerprint-first steps are unchanged, and the README and site snippets match line for line (only the HTML wrapper differs). The publisher already builds only binary-amd64 with Architectures=amd64, so the docs now match what the repository serves. The script parses with bash -n, cargo fmt --check is clean, and cargo test passes 47, fails none, with 1 ignored. I found no security issues.

## Summary

Committed as **`face134876`** — fixes FV-4QBCYS.

- Added `arch=amd64` to the identical sources lines in README and the publisher’s generated HTML.
- Added instructions for existing users to edit their sources file to remove the harmless i386 notice.
- Added an Unreleased changelog entry.
- Kept fingerprint-first verification unchanged. No version bump or release.

**Verified:** local publishing with a throwaway key and dummy `.deb`; all generated install blocks match README. Formatting and clippy passed; Rust tests: **47 passed, 1 ignored**.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `b64f63baf0` to `face134876`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
