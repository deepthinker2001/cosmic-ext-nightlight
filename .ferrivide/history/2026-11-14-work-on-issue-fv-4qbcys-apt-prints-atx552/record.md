# Work on issue FV-4QBCYS: apt prints a notice about i386 for our repository; add…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-4qbcys-apt-prints-a-notice-about-i386-for-our-r` · FV-4QBCYS: apt prints a notice about i386 for our repository; add arch=amd64 to the sources line

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
| `CHANGELOG.md` | modified | +4 −0 |  |
| `README.md` | modified | +6 −1 |  |
| `scripts/publish-apt.sh` | modified | +1 −1 |  |

## Tested

- `cargo fmt --all -- --check && cargo clippy --all-targets && cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Local publisher generated and signed the test repo; all three HTML install blocks match README. C2 checks passed: fmt/clippy clean, 47 Rust tests passed, 1 ignored.

## Summary

I’ll check the install snippets and publisher tests, add the amd64 filter without changing the fingerprint steps, then verify and commit the docs update.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `b64f63baf0` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
