# Work on issue FV-6BE3MK: Release: start on a v* tag push, no manual release step

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-6be3mk-release-start-on-a-v-tag-push-no-manual` · FV-6BE3MK: Release: a plain git push of main starts the release

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

Then:

> [Message from Project manager] Change of design for FV-6BE3MK. The user wants a plain `git push` of main (no tag, no `gh release create`) to start a release. So: trigger on push to the default branch (keep workflow_dispatch). A first job reads the version from Cargo.toml (cargo metadata --locked). If tag v<version> already exists, the run stops cleanly with a notice (no release, no failure), so ordinary pushes release nothing. If it doesn't exist, set TAG=v<version> and go on: build from that commit, then in the job holding the write token create the tag at that exact commit SHA and the release (gh release create --target <sha> --generate-notes, title = tag), then attach the .deb. Releasing = a merged commit that bumps the version in Cargo.toml (and Cargo.lock); we do the bump, the user only pushes. Drop the tag-matches-version check (tag is derived now). Keep the three-job split, pinned actions, minimal permissions (only the job that creates the tag/release gets contents: write), and the `release` environment on the signing job. NOTE: the `release` environment is limited to v* tags and main; make sure signing on a main push is allowed by that, and say in docs/RELEASING.md what the user must check in the environment's branch/tag rules. Update docs/RELEASING.md and README: release = bump version, merge, git push. Also check the apt job's gh-pages checkout and publish step still work from a main-push run. Check the YAML parses.
>
> [Issue FV-6BE3MK, as it stands now]
> FV-6BE3MK: Release: start on a v* tag push, no manual release step
> Status: Review
> Labels: Docs
> Origin: made in Ferrivide by Project manager
>
> Today .github/workflows/release.yml runs only when a GitHub release is published, so `git push` of a tag does nothing. Make `git tag vX.Y.Z && git push origin vX.Y.Z` enough.
>
> - Trigger on push of tags `v*` (keep workflow_dispatch). Drop the release:published trigger.
> - TAG comes from github.ref_name (or inputs.tag). Keep the tag-matches-Cargo-version check.
> - The attach job must create the release if it doesn't exist (gh release create --verify-tag --generate-notes, title = tag), then upload the .deb, so a release created by hand first also still works.
> - Keep the three-job split, pinned actions, the `release` environment (it already allows v* tags), and the minimal permissions: only attach gets contents: write.
> - Update docs/RELEASING.md (and README if it mentions the release step) to say: push a tag.
> - Check the YAML parses; no live run is possible.

> [Event] Quick reviewer (Haiku 5.5) asks for changes: The YAML parses, and the final commit (99dffa9) is internally consistent, but it doesn't do what the task asked. The task said to trigger on push of tags `v*` and keep the three-job split, with TAG from github.ref_name and the tag-matches-Cargo-version check kept. The engineer instead triggers on any push to `main` whose Cargo version has no tag, derives the tag from Cargo.toml, adds a fourth `plan` job, and drops the tag-matches-version check. A plain `git tag vX.Y.Z && git push origin vX.Y.Z` now does nothing, which is the bug the issue asked to fix. No build or test is affected, since only YAML and docs changed. The engineer should restore the tag-push trigger and the requested design, or get the user's agreement to the new design first.
> #1 [must fix] .github/workflows/release.yml:20: The trigger is `push: branches: [main]`, not `push: tags: ['v*']`. The task required that pushing a `v*` tag starts the release, and that the release:published trigger is dropped. Tag pushes now start nothing, so the issue's goal (`git tag vX.Y.Z && git push origin vX.Y.Z` is enough) is not met. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Restore `on: push: tags: ['v*']` alongside `workflow_dispatch`. Set TAG from `github.ref_name` on tag pushes (or `inputs.tag` on dispatch). Keep the run on the tagged commit. If the user prefers the untagged-version-on-main design, get that agreed first and say so in the task, rather than changing the design silently.
> #2 [must fix] .github/workflows/release.yml:73: The tag-matches-Cargo-version check that the task said to keep is gone (it was at c62b1d0 lines 51-58). The tag is now derived from Cargo.toml, so a manual `workflow_dispatch` with a mismatched `tag` input is never checked against the package version. Note that the `plan` job's `tag` is set from `INPUT_TAG` without any comparison. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Restore the check in `plan` (or `build`): when a tag comes from `github.ref_name` or `inputs.tag`, fail unless it equals `v$version`, as the old step did.
> #3 [should fix] .github/workflows/release.yml:42: The task asked to keep the three-job split. The workflow now has four jobs (`plan`, `build`, `attach`, `apt`). The split is a reasonable idea, but it isn't what was asked and it is not called out in the task's terms. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Fold the decision into the existing jobs, or get the user's agreement to a fourth job in the change description.
> #4 [should fix] docs/RELEASING.md:130: The docs now say to push `main` to release, and that the `release` environment must allow branch `main`. The task said the environment already allows `v*` tags and that the docs should say to push a tag. The environment rule change is not something the task asked for, and it is a manual GitHub step the user has to make. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Rewrite the Cut a release section to say: `git tag vX.Y.Z && git p…

(Shortened here; record.json has all of it.)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/release.yml` | modified | +73 −20 |  |
| `README.md` | modified | +1 −1 |  |
| `docs/RELEASING.md` | modified | +47 −31 |  |

## Tested

No test runs were reported.

## Summary

A push to `main` now starts the release workflow, and a version bump is all it takes to release. I couldn't run it on GitHub. I only checked that the YAML parses and that the job dependencies come out as expected.

**Environment rule to check:** the `release` environment must allow the branch `main`. A tag-only rule would have rejected the `apt` job, because a release now runs from `main`. `docs/RELEASING.md` says to check this under Settings → Environments → release → Deployment branches and tags. It also says to leave the `v*` tag rule in place, and not to block `github-actions` from creating `v*` tags in the tag ruleset. I haven't touched the real environment settings.

**Changes in `.github/workflows/release.yml`:**
- **Trigger:** it runs on push to `main`, and `workflow_dispatch` stays. The dispatch input is now an existing tag, and the run builds that tag's source.
- **New `plan` job:** it has a read-only token and no secrets. It reads the version with `cargo metadata --locked` and checks out with `fetch-tags`. If `v<version>` already exists, it prints a notice and the run ends cleanly with no release and no failure. Otherwise it outputs the tag and the commit SHA.
- **`build`:** it runs only for a release and builds exactly that SHA. I dropped the tag-matches-version check, since the tag is derived.
- **`attach`:** it is still the only job with `contents: write` besides `apt`. If no release exists for the tag, it runs `gh release create "$TAG" --target "$SHA" --generate-notes --title "$TAG"`. That makes the tag at that exact commit and the release in one call. Then it uploads the `.deb` and `.sha256`. An existing release is left alone, which is what a manual rerun relies on.
- **`apt`:** it needs `plan`, `build` and `attach`, so a non-release run skips it. Its gh-pages checkout and push don't depend on the trigger. On a `main` push the default-branch checkout is `main`, as before. The commit message now takes the tag from `plan`.

**Docs:** `docs/RELEASING.md` now describes the four jobs and the flow: bump the version in `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`, merge, then `git push`. It also covers recovery. If `build` fails, no tag exists yet, so fix `main` and push again. If the run fails after `attach` made the tag, rerun it with `gh workflow run release.yml --ref main -f tag=vX.Y.Z`. The README has one added line pointing to this flow.

The tag is created with the workflow's own token, so it doesn't start a second run.

Committed with `Fixes FV-6BE3MK`.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `c62b1d0632` to `99dffa9696`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
