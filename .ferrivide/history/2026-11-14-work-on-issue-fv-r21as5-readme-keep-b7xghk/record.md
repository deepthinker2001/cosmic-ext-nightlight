# Work on issue FV-R21AS5: README: keep it user-facing, move maintainer release…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-r21as5-readme-keep-it-user-facing-move-maintain` · FV-R21AS5: README: keep it user-facing, move maintainer release steps out

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-R21AS5: README: keep it user-facing, move maintainer release steps out
>
> The README is public and for people who want to use the applet. Keep in it: what it is, install (from the signed .deb release, and from source), how to add it to the panel, how to verify the download (with the key fingerprint), build from source, licence. Move the maintainer-only parts (creating the signing key, GitHub release environment and secrets, cutting a release, cargo-deb packaging details) into docs/RELEASING.md and drop them from the README, with at most one line linking to it. Keep release.yml working: it greps README.md for the fingerprint placeholder, so keep the placeholder where users read the fingerprint, or update the workflow's check and the README consistently. Fix any links and anchors that move. No code changes.
>
> Labels: Release

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `README.md` | modified | +22 −110 |  |
| `docs/RELEASING.md` | added | +98 −0 |  |

## Tested

No test runs were reported.

## Summary

(none given)

---

From `f571a7e98c` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
