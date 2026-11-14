# Work on issue FV-S0V7BV: Signed .deb release workflow

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-s0v7bv-signed-deb-release-workflow` · FV-S0V7BV: Signed .deb release workflow

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-S0V7BV: Signed .deb release workflow
>
> On a version tag, build the .deb in CI, sign it (detached GPG signature and a signed checksum file, or dpkg-sig, matching the user's other projects) and attach it to a GitHub Release. The key comes from repository secrets, never committed. List the secret names needed in the README.
>
> Labels: Release
>
> CI workflow already merged: follow its hardening (read-only permissions, persist-credentials false, actions pinned to full SHAs, dependabot). Use `just deb` / cargo install cargo-deb --locked. Signing key only via secrets.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | added | +93 −0 |  |
| `README.md` | modified | +26 −0 |  |

## Tested

No test runs were reported.

## Summary

Quick YAML check, then commit.

---

From `9d2c10e0b2` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
