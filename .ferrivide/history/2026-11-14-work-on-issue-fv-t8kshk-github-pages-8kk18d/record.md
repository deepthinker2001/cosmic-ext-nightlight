# Work on issue FV-T8KSHK: GitHub Pages site root returns 404: add an index page…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-t8kshk-github-pages-site-root-returns-404-add-a` · FV-T8KSHK: GitHub Pages site root returns 404: add an index page to the apt site

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-T8KSHK: GitHub Pages site root returns 404: add an index page to the apt site
>
> After a release, https://deepthinker2001.github.io/cosmic-ext-nightlight shows a 404. The site (built by scripts/publish-apt.sh onto gh-pages) has pool/, dists/, key.gpg and .nojekyll but no index.html, so the root has nothing to serve.
>
> Do:
> - Make scripts/publish-apt.sh write a small index.html at the site root: what the repo is, the apt setup lines (key download with fingerprint check, sources line; same as README, no shortcut that skips the fingerprint check), and links to key.gpg and the GitHub repo. Static, no scripts or external assets.
> - Keep the script's existing checks; test it locally with a throwaway key and dummy .deb (see project knowledge k12) and check index.html is in the output.
> - Update docs/RELEASING.md's list of gh-pages files.
> - Note in your report that the already-published site gets the page at the next release (or a manual workflow rerun from main with the existing tag), and that Settings → Pages must be set to gh-pages / root.
>
> Labels: Docs

Then:

> [Message from Project manager] Your work is committed and checked. Don't redo anything: call report_status now with the summary, files and test results, then stop so review can start.
>
> [Message from Project manager] Ignore if already done.

> Why aren't you working?

## Understood

Fix FV-T8KSHK by generating a static root index.html for the signed apt site, with the README's fingerprint-first setup instructions and project/key links, and documenting the generated file.

Assumptions:

- The landing page must pin the same trusted fingerprint as README.md rather than derive trust from the downloaded or signing key.
- Existing signing and fingerprint verification must remain unchanged.
- No live GitHub deployment or settings change is part of this task; the project manager owns requirements/features.

Plan:

- Add a quoted HTML heredoc after the publisher's signature checks.
- Update the gh-pages generated-file list and fingerprint maintenance note.
- Exercise publishing with a throwaway ed25519 key and dummy .deb, validate the generated HTML/instructions and negative key checks, and run contract-required Rust checks.
- Commit the verified change and report deployment prerequisites.

Out of scope:

- Changing release workflow or publishing to GitHub
- Using the real private signing key
- Changing README installation behavior or applet Rust code

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `docs/RELEASING.md` | modified | +6 −1 | List generated index.html, explain instruction/fingerprint synchronization and existing-site rollout. |
| `scripts/publish-apt.sh` | modified | +46 −0 | Write root index.html describing the applet, fingerprint-first apt setup, and key/source links after signature verification. |

### Decisions

- **Generate a literal static HTML page only after existing fingerprint/signature checks succeed.**: Supplies a root document without scripts, external assets, new dependencies, or executing the displayed shell commands; all existing security checks remain unchanged. Turned down: A separate site generator or workflow change; An unquoted heredoc that expands installation commands.
- **Pin the README's trusted fingerprint and preserve its separate verify-then-trust steps verbatim.**: Infrastructure/security: deriving the displayed expected fingerprint from whichever key signs would undermine the independent fingerprint comparison; local test confirms all displayed command blocks match README. Turned down: Derive expected fingerprint from exported key; Download directly into a trusted keyring.

## Tested

- `scripts/publish-apt.sh with scratch throwaway key/dummy .deb plus Python HTML assertions`: 5 passed, 0 failed
- `cargo fmt --all -- --check && cargo clippy --all-targets && cargo test --locked`: 47 passed, 0 failed, 1 skipped
- `cargo fmt --all -- --check && cargo clippy --all-targets && cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Local publisher checks passed: fresh publish and rebuild produce index.html, README commands match exactly, static links are present, and short/unknown signing keys are rejected.
- Contract checks passed: cargo fmt and clippy clean; Rust tests 47 passed, 0 failed, 1 ignored. Local apt publisher checks also passed.
- FV-T8KSHK committed as 04483c96d7: publisher generates static index.html with fingerprint-first README setup and links; all local checks pass. No live deployment performed: the existing site receives it at the next release or manual workflow rerun from main with an existing tag; Settings → Pages must use Deploy from a branch, gh-pages / (root).

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The script now writes a static index.html into the site root after the signature checks. It has the fingerprint-first apt steps, which match the README, and links to key.gpg and the GitHub repo. docs/RELEASING.md lists the new file and says when an already-published site gets the page. The commit message is clean. I read the diff and confirmed the script is already in the site directory when it writes the file, but I did not run the script or any tests.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): publish-apt.sh now writes a static index.html. It uses a quoted heredoc, so nothing in it is executed or expanded. The page has the README's fingerprint-first key check, the sources line, and links to key.gpg and the GitHub repo. The fingerprint and commands match the README. The existing checks are untouched, and docs/RELEASING.md lists index.html and the rollout caveat. I read the diff and ran `bash -n` on the script, but did not run the script or any tests. I did not check the commit messages against C4. No Rust code changed, so C2 doesn't apply.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change adds a static index.html, written from a quoted heredoc, so nothing in it is expanded or run while publishing. The page has no scripts and no external assets, and its apt steps match the README exactly: download to a temp file, check the fingerprint, and only then dearmor into a signed-by keyring. It adds no shortcut around the fingerprint check. The script's key and signature checks are unchanged and still run before the page is written. I ran it with a throwaway key and a dummy .deb: index.html is generated, and a short key ID is still rejected. If the signing key is rotated and the page's pinned fingerprint isn't updated, users are told to stop rather than trust the wrong key. Only a shell script and docs changed, so the Rust build and tests aren't affected.

## Summary

I've stopped because the work is finished. FV-T8KSHK is committed as `04483c96d7`, and the project manager asked me to stop there so review could start. Nothing is waiting on me; the next step is the reviewers'.

**What changed**
- **`scripts/publish-apt.sh`** now writes an `index.html` at the site root, so the Pages root has a page to serve instead of a 404. The page says what the repo is and gives the same apt setup steps as the README: download the key, check the fingerprint, then add the sources line. Those steps include the fixed fingerprint, so there's no shortcut that skips the check. It links to `key.gpg` and the GitHub repo. It's plain static HTML with no scripts and no external assets. The script only writes the page after its existing key and signature checks pass, and those checks are unchanged.
- **`docs/RELEASING.md`** now includes `index.html` in its list of gh-pages files. It also says the page's instructions and fingerprint must be kept matching the README.

**Testing**
- I ran the script locally with a throwaway key and a dummy `.deb`. It wrote `index.html` alongside the usual files.
- I checked that the page's command blocks match the README's exactly, that it has the key and repo links, and that it has no scripts or external assets.
- Short and unknown key IDs are still rejected before anything is written.
- `cargo fmt` and `cargo clippy` are clean, and `cargo test` passed 47 tests with 1 ignored.

**Still needed on GitHub (I haven't published anything)**
- The live site will only get the page at the next release, or if the release workflow is run by hand from `main` with the existing tag.
- Settings → Pages must be set to deploy from the `gh-pages` branch, folder `/ (root)`.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `6db778d492` to `04483c96d7`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
