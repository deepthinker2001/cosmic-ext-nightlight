# Work on issue FV-KRNV6Q: Release-page .deb has no signature users can verify…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-krnv6q-release-page-deb-has-no-signature-users` · FV-KRNV6Q: Release-page .deb has no signature users can verify (R14)

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-KRNV6Q: Release-page .deb has no signature users can verify (R14)
>
> Found by the release pass. Under R14, as the release job states it (release builds are signed, and users can verify the signatures; R14 isn't among the recorded requirements), the apt path is covered. The repository's `InRelease` and `Release.gpg` are signed, and apt checks the .deb's hash through them. The README's **From a release** path is not covered: the .deb on the GitHub release comes only with a `.sha256` attached to the same release. That catches a broken download, not a swapped file, because anyone who can change the asset can change its `.sha256` too.
>
> This was a deliberate trade-off: in FV-QVWSWT the user asked to drop the old gpgv verify-from-release flow when moving to the signed apt repository. So whether to sign the release-page download is the user's call.
>
> Options:
> - Have the `apt` job (which holds the key, and the write token for gh-pages) also make a detached `.deb.asc` (or a signed `SHA256SUMS`), and let `attach` or `apt` upload it. Then add a short `gpgv` step to the README's From a release section, using the key it already pins.
> - Or keep the release page as is, and say plainly in the README that only the apt repository is signed and that downloading from the release page relies on GitHub's TLS.
>
> Done when: the user has chosen, and the workflow and README match the choice.
>
> Labels: Docs

Then:

> [Event] Quick reviewer (Haiku 5.5) asks for changes: The branch fv-krnv6q-release-page-deb-has-no-signature-users has no commits beyond the reviewed base 626aa5b, and the worktree is clean. Nothing for FV-KRNV6Q is in the change: the release workflow still uploads only the .deb and its .sha256 (release.yml lines 170-174), and README.md "From a release" (lines 131-141) is unchanged. The task also needs the user's choice between the two options; no choice is recorded, so the done condition isn't met.
> #1 [must fix] README.md:131: No work is committed on the branch: the diff against the base is empty, so the release-page path is unchanged and still has no signature. Either the work was never committed, or the agent stopped before starting it. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Have the engineer make the change on the branch. First record the user's choice between the two options in the task (a detached signature from the apt job, or a README note that only the apt repository is signed). Then update release.yml and the README to match that choice, and commit.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> Answering your question "How should the release-page .deb be handled?":
>
> Sign it (.deb.asc + README gpgv step)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +16 −0 |  |
| `README.md` | modified | +19 −2 |  |

## Tested

No test runs were reported.

Progress reports:

- Waiting for the user to choose between signing the release-page .deb and documenting that only apt is signed; nothing is committed yet.

## Summary

Add the detached signature in the same signing step, and upload it to the release.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `626aa5bb40` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
