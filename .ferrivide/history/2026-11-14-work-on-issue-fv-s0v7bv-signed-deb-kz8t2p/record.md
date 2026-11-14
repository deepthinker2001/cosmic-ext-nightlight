# Work on issue FV-S0V7BV: Signed .deb release workflow

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-s0v7bv-signed-deb-release-workflow` · FV-S0V7BV: Signed .deb release workflow

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-S0V7BV: Signed .deb release workflow
>
> On a version tag, build the .deb in CI, sign it (detached GPG signature and a signed checksum file, or dpkg-sig, matching the user's other projects) and attach it to a GitHub Release. The key comes from repository secrets, never committed. List the secret names needed in the README.
>
> Labels: Release
>
> CI workflow already merged: follow its hardening (read-only permissions, persist-credentials false, actions pinned to full SHAs, dependabot). Use `just deb` / cargo install cargo-deb --locked. Signing key only via secrets.

Then:

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The workflow keeps the CI hardening: actions pinned to SHAs, persist-credentials false, a read-only default, and tag names passed through env with no template injection. But it signs in the same job and on the same runner that just compiled and ran third-party code (build scripts, proc macros, and unpinned cargo-install tools), with a restored build cache and the contents: write token in the job. Anything that runs during the build can wait on the runner and take the private key and passphrase in the Sign step. The README's verify steps also let a swapped, unsigned SHA256SUMS pass, and they don't name the key to trust. cargo test: 9 passed, 1 ignored; the YAML parses.
> #1 [must fix] .github/workflows/release.yml:15: Building, signing and publishing all happen in one job. Earlier steps in that job run untrusted code: cargo install of unpinned crates, every dependency's build.rs and proc macros, and code from a restored cache. That code runs as the same user on the same runner, so it can leave something behind that the Sign step then runs: a gpg wrapper earlier in PATH, or changes under ~/.cargo/bin or $HOME. It can also watch /proc while the Sign step runs and read GPG_PRIVATE_KEY and GPG_PASSPHRASE from its environment. The job's write permission is also wider than the build needs. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Split it into three jobs. `build`: permissions contents: read, no secrets; runs `just deb` and uploads target/debian/*.deb with actions/upload-artifact (pinned SHA). `sign`: needs build, contents: read, `environment: release`; no checkout, no cache, no cargo; uses the preinstalled /usr/bin/gpg by absolute path; downloads the artifact, signs it, and uploads the signed set. `publish`: needs sign, contents: write; only runs `gh release create`.
> #2 [must fix] .github/workflows/release.yml:48: The release job restores Swatinem/rust-cache, and any run on the default branch may have written that cache. A poisoned target/ or ~/.cargo, from a compromised dependency or a malicious build script on main, then gets into the signed artifact and the job that holds the signing key. This is the cache-poisoning pattern: release builds should never use shared caches. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Remove the rust-cache step from release.yml, so the release is built clean. If you keep a cache, set `save-if: false` and only use it in the secret-free build job after the split; clean is still better.
> #3 [must fix] .github/workflows/release.yml:51: `cargo install --locked just cargo-deb` installs whatever versions are newest at release time, and they run in the release job. A compromised new release of either crate, or of one of their dependencies, would run with the key still to come in this job. Dependabot doesn't track these crates. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Pin exact versions, e.g. `cargo install --locked just@<x.y.z> cargo-deb@<x.y.z>`, matching the version …

(Shortened here; record.json has all of it.)

> [Event] Your other reviewers' findings, to fix along with the changes asked for:
> #7 [should fix] README.md:85: The verify commands need the public key, but the README never says where to get it or what its fingerprint is. A user who downloads the .deb and runs `gpg --verify` gets 'no public key' and can't check the signature. Nothing in the workflow publishes the public key either. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Publish the armored public key (for example as a `cosmic-ext-nightlight-release-key.asc` file in the repo or attached to each release) and add to the README: the import command (`gpg --import <file>`) and the key's full fingerprint, so the user can compare it before trusting the signature.
> #8 [suggestion] .github/workflows/release.yml:68: `rm -rf "${GNUPGHOME}"` runs only on success. If a step before it fails under `set -e`, the temporary keyring with the private key stays on the runner's disk. The runner is ephemeral, so the risk is small. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Remove the directory with a trap: `trap 'rm -rf "${GNUPGHOME}"' EXIT` right after the mktemp line.
> #9 [suggestion] .github/workflows/release.yml:55: `cargo install --locked just cargo-deb` installs whatever versions are newest on each release, so the build tools aren't pinned. The task said to use `cargo install cargo-deb --locked`, so this is as asked, but it can change a signed release's build without any repo change. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Pin the versions, for example `cargo install --locked just@<ver> cargo-deb@<ver>`, and bump them in a reviewed change. Or leave it and note the trade-off in the README.
> #10 [suggestion] .github/workflows/release.yml:46: Swatinem/rust-cache restores a cache that CI branches can write, and this job builds and signs the release artifact. A poisoned cache could end up in a signed .deb. Drop the cache step from this job, or set `save-if: false`. (Sonnet 5.5)
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> [Event] Cyber reviewer (Opus 5.5) asks for changes: #1–#4, #6 and #7 are fixed. Build, sign and publish now run as separate jobs, with no cache, pinned tools, environment secrets, a cleanup trap, and the key published with a fingerprint slot in the README. #5 is only partly fixed: the new verify command still passes a tampered checksum file. I reproduced it locally: with a BAD signature, `gpg --decrypt` still writes the forged text, and the pipeline exits 0 with "OK".
> #1 [must fix] README.md:84: `gpg --decrypt SHA256SUMS.asc | sha256sum -c --ignore-missing -` doesn't stop on a bad signature. gpg writes the clear-signed text to stdout even when the signature is BAD or the key is unknown, and the pipeline's exit status is sha256sum's. I tested it: I edited a hash in SHA256SUMS.asc to match a swapped .deb, gpg printed 'BAD signature', and the command still printed 'a.deb: OK' and exited 0. So a forged SHA256SUMS.asc with a matching swapped .deb passes, which is the attack #5 was about. The only guard is the sentence asking users to read gpg's stderr. The command also accepts any key in the user's keyring. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Verify against the release key alone, and hash-check only if that succeeds. For example: `gpg --dearmor < signing-key.asc > signing-key.gpg` once, after checking the fingerprint. Then `gpgv --keyring ./signing-key.gpg --output SHA256SUMS.verified SHA256SUMS.asc && sha256sum -c --ignore-missing SHA256SUMS.verified`. I checked that gpgv exits 1 on the tampered file, so `&&` stops the hash check. Add a note that SHA256SUMS.verified must not be used if gpgv failed. Do the same for the .deb: `gpgv --keyring ./signing-key.gpg cosmic-ext-nightlight_*.deb.asc cosmic-ext-nightlight_*.deb`.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> [Event] Your other reviewers' findings, to fix along with the changes asked for:
> #2 [should fix] README.md:73: The fingerprint the README tells users to compare against is still the placeholder `FINGERPRINT-TO-BE-FILLED-IN-BY-THE-MAINTAINER-BEFORE-THE-FIRST-RELEASE`. Round-1 finding #5 asked for the real fingerprint, so the verify steps can't be checked yet. This must be filled in before the first tag is pushed. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Once the signing key exists, put its full 40-character fingerprint in that code block. Until then, keep the placeholder, and check the README is filled in before the first release.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | added | +148 −0 |  |
| `README.md` | modified | +60 −0 |  |

## Tested

No test runs were reported.

Progress reports:

- Release workflow split into build/sign/publish; README verifies with gpgv against the release key; build fails until the fingerprint placeholder is filled. Not run on GitHub.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Round 3 check. Finding #1 is fixed: the README now verifies with `gpgv` against a dearmored release key, and the hash check runs only after `gpgv` succeeds. I tested it with a throwaway key. A genuine checksum file and .deb pass. A checksum file signed by an unknown key makes gpgv exit 2, so the hash check never runs. A tampered .deb makes gpgv exit 1. Finding #2 is the placeholder fingerprint. The engineer's reason (the maintainer has to create the signing key, which needs a user decision) holds, and the build job now fails any tag push while the placeholder is still in the README, so no release can go out before it's filled in. I'm accepting that reason, so it goes to a backlog issue.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): #1 is fixed: the README verifies with `gpgv` against the release key alone, and the hash check only runs if `gpgv` succeeds. #2's reason is accepted. The fingerprint depends on a key only the maintainer has, and the release's build job now refuses to run while the placeholder is in the README.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): #1 is fixed. The README now checks the hashes only after `gpgv --keyring ./signing-key.gpg` passes, against the release key alone, and the `.deb` signature is checked the same way. This matches the command I tested last round: `gpgv` fails on the tampered file, so the hash check never runs. I accept the deferral of #2, since the fingerprint depends on a key only the user holds. The new build step fails any release while the placeholder is still in the README, so no tag can publish without it. The new lines raise no new security issue.

## Summary

I fixed the cyber reviewer's remaining finding (#5 on the README's verification), and committed it. I couldn't fill in the signing key's fingerprint (#2), so I'm declining that one. Nothing has run on GitHub.

- **#5, verification:** the README now verifies against the release key alone with `gpgv`, so a bad signature stops the check. You first check the fingerprint of `signing-key.asc` and convert it with `gpg --dearmor`. Then `gpgv … --output SHA256SUMS.verified SHA256SUMS.asc && sha256sum -c --ignore-missing SHA256SUMS.verified` checks the hashes, and `gpgv` checks the `.deb` the same way. The README warns not to use `SHA256SUMS.verified` or the `.deb` if `gpgv` fails. I didn't run these commands myself; they follow the cyber reviewer's suggested fix.
- **#2, fingerprint:** I'm declining it as needs_user_decision, because the real 40-character fingerprint belongs to a key you choose or create. The README keeps the placeholder. I added a first step to the `build` job in `release.yml` that fails while the placeholder is still in `README.md`, so no release can go out without the fingerprint.

Before the first release, a maintainer has to put the real fingerprint in the README. They also have to set up the `release` environment and the tag ruleset described there.

---

From `9d2c10e0b2` to `1c7e89d5c1`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
