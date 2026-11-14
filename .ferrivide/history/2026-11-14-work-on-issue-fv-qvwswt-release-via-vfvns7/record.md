# Work on issue FV-QVWSWT: Release via signed APT repo on gh-pages, matching…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-qvwswt-release-via-signed-apt-repo-on-gh-pages` · FV-QVWSWT: Release via signed APT repo on gh-pages, matching cosmic-ext-dimmer

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

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +84 −88 |  |
| `README.md` | modified | +27 −32 |  |
| `docs/RELEASING.md` | modified | +83 −55 |  |
| `scripts/publish-apt.sh` | added | +51 −0 |  |

## Tested

No test runs were reported.

## Summary

Also fix the "secret and variable" heading reference if any, then commit.

---

From `4b7f92a194` to `011f177694`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
