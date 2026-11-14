# Work on issue FV-QVWSWT: Release via signed APT repo on gh-pages, matching…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-qvwswt-release-via-signed-apt-repo-on-gh-pages` · FV-QVWSWT: Release via signed APT repo on gh-pages, matching cosmic-ext-dimmer

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-QVWSWT: Release via signed APT repo on gh-pages, matching cosmic-ext-dimmer
>
> Replace the current tag-triggered signed-.deb release with the same pattern as /home/mad/projects/cosmic-ext-dimmer (read it with request_access; read-only, copy and adapt, don't change it): .github/workflows/release.yml (on release published + workflow_dispatch; build job attaches .deb and .sha256 to the release; apt job signs into an APT repo on the gh-pages branch with scripts/publish-apt.sh), the scripts, and whatever its docs/README say about the gh-pages branch setup, repo layout, and how users add the repo. Use secret APT_SIGNING_KEY and variable APT_SIGNING_KEY_ID, same as dimmer. Adapt names to cosmic-ext-nightlight (package, .deb name, tag check). Keep this project's existing hardening (pinned action SHAs, split jobs, no cache in release, pinned tool versions, persist-credentials false). Update README (user-facing: add the apt repo with the signing key, apt install, how to update; drop the manual gpgv verify-from-Release flow and the fingerprint placeholder together with the workflow's placeholder check, consistently) and docs/RELEASING.md (maintainer: one-time gh-pages branch creation, secret and variable, cutting a release by publishing a GitHub release). Say plainly what can't be verified without GitHub.
>
> Labels: Release
>
> Request read access to /home/mad/projects/cosmic-ext-dimmer with request_access (the user named it) to copy its release workflow, scripts/publish-apt.sh and APT docs.

Then:

> [Event] Quick reviewer (Haiku 5.5) asks for changes: The change does what the task asked: the release workflow matches cosmic-ext-dimmer's build/apt split, publish-apt.sh is a faithful copy with the names adapted, the README drops the gpgv flow and the placeholder check, and RELEASING.md covers the one-time setup. I checked the script with a throwaway key and a real .deb: it builds the pool, Packages, Release, InRelease and Release.gpg, and the signature verifies. The one thing to fix is that the signing job lost its environment gate that the project had before.
> #1 [must fix] .github/workflows/release.yml:87: The `apt` job no longer runs in the `release` environment. The old `sign` job had `environment: release` (required reviewer, deployment limited to v* tags, secret kept as an environment secret), and RELEASING.md used to set that up. Now every published release signs with APT_SIGNING_KEY automatically, with no approval step, and the secret is a repository secret that any workflow in the repo can read. The task asked to keep this project's existing hardening, and dimmer's lack of an environment doesn't make dropping it a deliberate choice here. Fix: add `environment: release` to the `apt` job, keep APT_SIGNING_KEY and APT_SIGNING_KEY_ID as environment secret and variable in RELEASING.md's setup (as before), and state the approval gate in the doc. If the engineer drops the gate on purpose, say why in the change and have the cybersecurity stage review it. (Quick reviewer (Haiku 5.5))
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The fix for last round's finding is in: the `apt` job runs in the `release` environment again, and the key is an environment secret. But the `build` job now holds a `contents: write` token while it runs untrusted build code (dependency build scripts, `cargo install`). The old workflow kept that job read-only and used a separate publish job that ran no cargo. With that token, a compromised dependency could push to `main` and change `scripts/publish-apt.sh`, which the approved `apt` job then runs with the signing key loaded. That would expose the key. It could also swap the release's `.deb` before `apt` signs it. No Rust source changed; `sh -n` passes on the script; shellcheck and actionlint aren't installed here.
> #1 [must fix] .github/workflows/release.yml:38: The `build` job runs `cargo install` and `just deb`, which execute code from every dependency, with `contents: write` on the job token. The old workflow gave this job `contents: read`, and only a publish job that ran no cargo held the write token. The task said to keep that hardening. A malicious build script can read the token from the runner even with persist-credentials false; on GitHub-hosted runners, job tokens can be scraped from Runner.Worker memory, as the tj-actions attack did. With it, the script can do three things. It can push to the default branch, editing `scripts/publish-apt.sh`, which the `apt` job checks out from that branch (line 96) and runs with APT_SIGNING_KEY in GNUPGHOME, so the key can be stolen. The environment approval doesn't help, because the reviewer approves a run, not the script's contents. It can replace the release assets that `apt` downloads and signs; the `.sha256` check proves nothing there, since it comes from the same release. And it can push gh-pages directly. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Give `build` `permissions: contents: read`. Have it upload `target/debian/*.deb` with a pinned actions/upload-artifact (as the old workflow did) and expose the sha256 as a job output. Add an `attach` job with `needs: build` and `contents: write`, with no checkout and no cargo, that downloads the artifact and runs `sha256sum` and `gh release upload`. Have `apt` take the .deb from the artifact (or check the downloaded asset against the build job's output hash), not from the release alone. Also state in RELEASING.md that `main` must have branch protection (no direct pushes, reviews required), because `apt` runs `main`'s publish script with the key loaded.
> #2 [should fix] .github/workflows/release.yml:101: The gh-pages checkout keeps its write credentials in `site/.git/config` through the signing step. That step runs `apt-ftparchive` over .deb files downloaded from the release, which the step should treat as untrusted, so the token is exposed for longer than it needs to be. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Check gh-pages out with `persist-credentials: false`. In the final step, push with the …

(Shortened here; record.json has all of it.)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +111 −72 |  |
| `README.md` | modified | +27 −32 |  |
| `docs/RELEASING.md` | modified | +96 −56 |  |
| `scripts/publish-apt.sh` | added | +51 −0 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Both earlier findings are fixed. #1: `build` now has `contents: read`, hands the .deb over as an artifact, and exposes its hash as an output. A new `attach` job holds the release write token with no checkout and no cargo. `apt` takes the .deb from the artifact and checks it against the build's hash, and RELEASING.md gained a "Protect main" section. #2: the gh-pages checkout has `persist-credentials: false`, and the push passes the token only in the publish step through `http.extraheader`. The workflow parses, and the job permissions are as described. The two artifact action SHAs were not checked against their release tags, since there is no network here. That check, and the rest of the GitHub behaviour, can only be done on GitHub.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Both findings are fixed. `build` now has `contents: read` and hands over the .deb as an artifact plus a hash output. A separate `attach` job, with no checkout and no cargo, holds the write token. `apt` signs the artifact after checking it against the build's hash, not the release asset. The gh-pages checkout uses `persist-credentials: false`, and the token is passed only to the push. RELEASING.md now says `main` and `gh-pages` need branch protection. I only read the diff; nothing here can be tested without GitHub.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): Both findings from the last round are fixed. #1: `build` now has a read-only token and passes the .deb on as a pinned artifact, along with its hash. A separate `attach` job, which runs no checkout and no cargo, holds the release write token. `apt` signs the build's artifact after checking it against the build's hash, and RELEASING.md now requires branch protection on `main` and `gh-pages`. #2: gh-pages is checked out with `persist-credentials: false`, and the token is passed only to the final `git push`. Nothing new in the changed lines needs fixing. No Rust source changed in this round.

## Summary

I fixed both findings and committed, but I haven't run the workflow on GitHub. I did check that the YAML parses and that each job has the intended `needs` and `permissions`.

**#1 (write token in the build job).** `.github/workflows/release.yml` now has three jobs:
- **`build`** has a read-only token and no secrets. It runs cargo and `just deb`, then hands the `.deb` over as an artifact (pinned `upload-artifact`) along with its sha256 as a job output.
- **`attach`** holds the release write token. It has no checkout and no cargo. It downloads the artifact, checks it against the build's hash, and uploads the `.deb` and its `.sha256` to the release.
- **`apt`** runs after both, in the `release` environment. It signs the build job's artifact, not the asset on the release, and checks it against the build's hash first.

**#2 (gh-pages credentials).** The `gh-pages` checkout now uses `persist-credentials: false`. Only the final publish step gets the token, and it passes it to `git push` through `http.extraheader`.

**`docs/RELEASING.md`.** It now describes the three jobs. A new "Protect `main`" section says `main` needs branch protection (no direct pushes, reviews required), because `apt` runs `main`'s `publish-apt.sh` with the key loaded. It also says `gh-pages` should only accept pushes from the workflow.

**Still true.** A compromised build could still hand over a bad `.deb` that `apt` then signs. That's inherent, but it can no longer touch `main`, the release, or the key.

---

From `4b7f92a194` to `5ee48e32bb`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
