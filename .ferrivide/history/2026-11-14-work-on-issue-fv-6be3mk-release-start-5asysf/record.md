# Work on issue FV-6BE3MK: Release: start on a v* tag push, no manual release step

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-6be3mk-release-start-on-a-v-tag-push-no-manual` · FV-6BE3MK: Release: start on a v* tag push, no manual release step

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-6BE3MK: Release: start on a v* tag push, no manual release step
>
> Today .github/workflows/release.yml runs only when a GitHub release is published, so `git push` of a tag does nothing. Make `git tag vX.Y.Z && git push origin vX.Y.Z` enough.
>
> - Trigger on push of tags `v*` (keep workflow_dispatch). Drop the release:published trigger.
> - TAG comes from github.ref_name (or inputs.tag). Keep the tag-matches-Cargo-version check.
> - The attach job must create the release if it doesn't exist (gh release create --verify-tag --generate-notes, title = tag), then upload the .deb, so a release created by hand first also still works.
> - Keep the three-job split, pinned actions, the `release` environment (it already allows v* tags), and the minimal permissions: only attach gets contents: write.
> - Update docs/RELEASING.md (and README if it mentions the release step) to say: push a tag.
> - Check the YAML parses; no live run is possible.
>
> Labels: Docs

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +13 −6 |  |
| `docs/RELEASING.md` | modified | +9 −7 |  |

## Tested

No test runs were reported.

## Summary

(none given)

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `c62b1d0632` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
