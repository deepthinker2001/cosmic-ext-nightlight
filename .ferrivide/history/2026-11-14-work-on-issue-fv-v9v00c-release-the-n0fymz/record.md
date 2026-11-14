# Work on issue FV-V9V00C: Release: the GitHub `release` environment rejects v*…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-v9v00c-release-the-github-release-environment-r` · FV-V9V00C: Release: the GitHub `release` environment rejects v* tags, so nothing has ever been signed

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-V9V00C: Release: the GitHub `release` environment rejects v* tags, so nothing has ever been signed
>
> Found by the release pass, from the public GitHub Actions API.
>
> - **v0.1.0** (run 43556248562): `build` and `attach` passed, so the unsigned .deb and .sha256 are on the release. `apt` failed at once: "Tag "v0.1.0" is not allowed to deploy to release due to environment protection rules." So `gh-pages` still holds only its empty first commit, `key.gpg` isn't served, and the README's apt install instructions fail (the key download 404s).
> - **v0.1.1** (run 43560385933): `build` failed at "Check the tag matches the version". The tag points at ea00505, where `Cargo.toml` still says 0.1.0. No package was produced, and the tag is now spent.
>
> What the user has to do (GitHub settings; it can't be done from a branch):
> 1. Settings → Environments → `release` → Deployment branches and tags: add `v*` as a rule of type **Tag** (and keep `main`). docs/RELEASING.md now says this explicitly.
> 2. For the next release, set `version` in `Cargo.toml` to a new version (not 0.1.0 or 0.1.1), move CHANGELOG.md's Unreleased section under it, and publish the release.
> 3. Optionally rerun the packaging for v0.1.0 (`gh workflow run release.yml -f tag=v0.1.0`) so apt users get 0.1.0, or skip straight to the new version.
> 4. After the first `apt` run, check from a clean machine that `key.gpg`'s fingerprint matches the README's (`0472 CEF0 1CE0 E79E C118 662D 34A0 6EB9 79E5 8584`). publish-apt.sh now also checks `key.gpg` against `APT_SIGNING_KEY_ID` and verifies the signatures with gpgv.
>
> Labels: Docs
>
> Prepare release 0.2.0 (0.1.0 and 0.1.1 tags are spent). Set version = "0.2.0" in Cargo.toml and update Cargo.lock to match; move CHANGELOG.md's Unreleased section under a 0.2.0 heading with today's date (2026-11-13); check docs/RELEASING.md matches. Run cargo check --locked and the tests. Do not tag or publish; the user does that.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `CHANGELOG.md` | modified | +4 −2 |  |
| `Cargo.lock` | modified | +1 −1 |  |
| `Cargo.toml` | modified | +1 −1 |  |

## Tested

No test runs were reported.

## Summary

Those are other crates, so I'm leaving them as they are.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `41ea815b93` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
