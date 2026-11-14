# Work on issue FV-KRNV6Q: Release-page .deb has no signature users can verify…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-krnv6q-release-page-deb-has-no-signature-users` · FV-KRNV6Q: Release-page .deb has no signature users can verify (R14)

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

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The branch now signs the release-page .deb in the apt job with the release-environment key, checks the signature with gpgv against the key.gpg that publish-apt.sh already checked, uploads the .asc, and documents a gpgv check. No secret is exposed: the key stays in a temporary GNUPGHOME that is removed afterwards, and the token is github.token. One thing must change first: the README's verify-and-install block still installs the .deb when the signature check fails, so the new check doesn't stop a swapped file.
> #1 [must fix] README.md:151: In the 'verify and install' block, `gpgv`, `sha256sum -c` and `sudo apt install` sit on separate lines. When a user pastes the block into a shell, a BAD signature or failed hash doesn't stop the next line, so a swapped .deb is installed as root anyway. The line "`gpgv` must say `Good signature`" comes after the block, which is too late. The signature exists to defend against exactly this attack, and the block defeats it. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Chain the commands so that installing depends on the checks passing, e.g. ```sh gpg --dearmor -o "$dir/key.gpg" "$dir/key.asc" \ && gpgv --keyring "$dir/key.gpg" cosmic-ext-nightlight_*.deb.asc cosmic-ext-nightlight_*.deb \ && sha256sum -c cosmic-ext-nightlight_*.deb.sha256 \ && sudo apt install ./cosmic-ext-nightlight_*.deb rm -r "$dir" ``` and move the 'must say Good signature; if not, stop' note before the block.
> #2 [should fix] .github/workflows/release.yml:232: `gpg --batch --dearmor -o "$GNUPGHOME/check.gpg" site/key.gpg` runs inside the per-.deb loop without `--yes`. On a second .deb it exits 2 ('dearmoring failed: File exists', which I reproduced), and the release step fails. There's only one .deb today, but the loop is written for several. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Dearmor once, before the `for` loop (or add `--yes`), and keep only the sign and gpgv lines inside the loop.
> #3 [suggestion] README.md:151: The globs `cosmic-ext-nightlight_*.deb.asc cosmic-ext-nightlight_*.deb` expand to several arguments when more than one version is in the folder. gpgv then treats the extra files as data and reports a confusing BAD signature (it fails safe), and `apt install ./…*.deb` would install several files. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Tell users to run it in a folder holding just the one download, or set `deb=cosmic-ext-nightlight_X.Y.Z_amd64.deb` and use "$deb" / "$deb.asc" / "$deb.sha256".
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an …

(Shortened here; record.json has all of it.)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +16 −0 | The apt job makes a detached .deb.asc with the apt signing key. It checks it with gpgv against the published key.gpg, then uploads it to the release. |
| `README.md` | modified | +26 −4 | From a release now tells users to check the key fingerprint, run gpgv on the .asc, and then the .sha256. |

## Tested

No test runs were reported.

Progress reports:

- Waiting for the user to choose between signing the release-page .deb and documenting that only apt is signed; nothing is committed yet.
- Per the user's choice, the release job now signs the release-page .deb, and the README has a gpgv step. The workflow hasn't run; I only checked that the YAML parses.
- I fixed all three review findings and committed them. Nothing has run on GitHub; the only check was that the workflow YAML parses.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): All three earlier findings are fixed in commit 99a6edc. The README's install block now chains the dearmor, gpgv, sha256sum and apt install with &&, so nothing installs unless the signature and hash both pass, and it uses a `deb` variable instead of globs. The workflow dearmors the key once, before the loop, with --yes. The workflow still parses. The new commit message is about the change and names no agent, which meets C4. No Rust code changed, so C2 doesn't apply. As before, your confirmation that you chose the detached signature is still needed before merge.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): All three earlier findings are fixed. The README chains gpgv, sha256sum and apt install with `&&` and uses a `deb` variable. The workflow dearmors the key once with `--yes`, before the loop. The commit message names no agents (C4).

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): All three findings are fixed. In the README, the gpgv, sha256sum and apt install commands are now joined with &&, so nothing is installed if a check fails. The commands name one file through $deb instead of a glob, and the warning comes before the commands. The workflow converts the key once, before the signing loop, with --yes. The new commit message names no agents (C4), and the changed lines raise no new security issues.

## Summary

I fixed all three review findings and committed them. I haven't run the workflow on GitHub. The only check was that `release.yml` parses as YAML.

- **#1 (README):** The README's verify-and-install block chains `gpg --dearmor`, `gpgv`, `sha256sum -c` and `sudo apt install` with `&&`. A bad signature or hash now stops the install. The "must say Good signature, otherwise don't install" note is now part of the same section. I also added the cleanup (`rm -r "$dir"`) to the wrong-fingerprint case.
- **#3 (README):** The block now sets `deb=cosmic-ext-nightlight_X.Y.Z_amd64.deb`, where you fill in the real version. It uses `"$deb"`, `"$deb.asc"` and `"$deb.sha256"` instead of globs. That way `gpgv` and `apt install` only ever see the one file.
- **#2 (`.github/workflows/release.yml`):** `gpg --dearmor` now runs once, with `--yes`, before the signing loop. Only the sign and `gpgv` lines stay inside the loop, so a second .deb wouldn't fail on an existing file.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `626aa5bb40` to `99a6edc099`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
