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

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +82 −89 |  |
| `README.md` | modified | +27 −32 |  |
| `docs/RELEASING.md` | modified | +74 −55 |  |
| `scripts/publish-apt.sh` | added | +51 −0 |  |

## Tested

No test runs were reported.

## Summary

Smoke-test the script with a throwaway key and a dummy package.

---

From `4b7f92a194` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
