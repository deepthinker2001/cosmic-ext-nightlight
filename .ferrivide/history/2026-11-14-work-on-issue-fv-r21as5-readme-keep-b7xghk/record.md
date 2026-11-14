# Work on issue FV-R21AS5: README: keep it user-facing, move maintainer release…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-r21as5-readme-keep-it-user-facing-move-maintain` · FV-R21AS5: README: keep it user-facing, move maintainer release steps out

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-R21AS5: README: keep it user-facing, move maintainer release steps out
>
> The README is public and for people who want to use the applet. Keep in it: what it is, install (from the signed .deb release, and from source), how to add it to the panel, how to verify the download (with the key fingerprint), build from source, licence. Move the maintainer-only parts (creating the signing key, GitHub release environment and secrets, cutting a release, cargo-deb packaging details) into docs/RELEASING.md and drop them from the README, with at most one line linking to it. Keep release.yml working: it greps README.md for the fingerprint placeholder, so keep the placeholder where users read the fingerprint, or update the workflow's check and the README consistently. Fix any links and anchors that move. No code changes.
>
> Labels: Release

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [suggestion] README.md:24: The install section ends with 'To build from source instead, read on.', but the 'Verify a download' section sits between the install steps and the build section, so 'read on' sends the reader through verification first. Reword it to point at the section by name, e.g. 'To build from source instead, see Build and install from source below.' (Quick reviewer (Haiku 5.5))
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `README.md` | modified | +23 −110 |  |
| `docs/RELEASING.md` | added | +98 −0 |  |

## Tested

No test runs were reported.

## Summary

I moved the maintainer-only parts of `README.md` into a new `docs/RELEASING.md` and committed. Nothing was built or run: no workflow run and no link check. I did grep the workflow, and the placeholder it looks for is still in the README.

- **README:** it now covers only what users need: what it is, requirements, installing the signed `.deb`, verifying the download with the key fingerprint, building and installing from source, adding it to the panel, and the licence. The last line links to `docs/RELEASING.md`.
- **`docs/RELEASING.md`:** it holds the `cargo-deb` packaging details, how a release is built, creating the signing key, the GitHub `release` environment and secrets, and cutting a release.
- **Workflow:** I left `release.yml` alone. It greps `README.md` for `FINGERPRINT-TO-BE-FILLED-IN`, and that placeholder is still in the README's fingerprint block, so the check works as before. `RELEASING.md` says to keep it there until the real fingerprint is pasted in.
- **Links and anchors:**
  - The README's "Verify a download" heading is now a top-level section. I kept its anchor, `#verify-a-download`, so the install steps still resolve.
  - The links inside `RELEASING.md` use its own new section anchors and `../README.md#verify-a-download`.
- **Wording changes you didn't ask for:**
  - I moved the "build tools" requirement into a "to build from source" bullet.
  - I renamed "Build and install" to "Build and install from source".
  - I moved the `sudo apt remove` line into the install section.

---

From `f571a7e98c` to `c25e74dfa2`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
