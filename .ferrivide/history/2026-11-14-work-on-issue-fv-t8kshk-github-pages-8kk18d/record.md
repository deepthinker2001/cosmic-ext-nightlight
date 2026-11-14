# Work on issue FV-T8KSHK: GitHub Pages site root returns 404: add an index page…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-t8kshk-github-pages-site-root-returns-404-add-a` · FV-T8KSHK: GitHub Pages site root returns 404: add an index page to the apt site

0 tokens in, 0 out

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
| `docs/RELEASING.md` | modified | +6 −1 |  |
| `scripts/publish-apt.sh` | modified | +46 −0 |  |

## Tested

- `scripts/publish-apt.sh with scratch throwaway key/dummy .deb plus Python HTML assertions`: 5 passed, 0 failed
- `cargo fmt --all -- --check && cargo clippy --all-targets && cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Local publisher checks passed: fresh publish and rebuild produce index.html, README commands match exactly, static links are present, and short/unknown signing keys are rejected.
- Contract checks passed: cargo fmt and clippy clean; Rust tests 47 passed, 0 failed, 1 ignored. Local apt publisher checks also passed.

## Summary

The local publish test passed: `index.html` is generated, its command blocks match the README exactly, and both repository signatures verify. Short and unknown key IDs are still rejected. Formatting, Clippy, and Rust tests also passed (47 passed, 1 ignored). I’ll review the diff and commit.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `6db778d492` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
