# Work on issue FV-W86KWZ: Add third-party notices and a list of bundled assets…

**In progress** 2026-11-14 · Engineer 2 (project agent) · `claude-sonnet-5-5` · branch `fv-w86kwz-add-third-party-notices-and-a-list-of-bu` · FV-W86KWZ: Add third-party notices and a list of bundled assets to the package

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-W86KWZ: Add third-party notices and a list of bundled assets to the package
>
> Found by the release pass. The project has no third-party notices file and no list of bundled assets, and the .deb ships only the binary, the .desktop file and two icons. The binary embeds third-party fonts and code whose licenses require their notices to travel with copies:
>
> - **Fonts compiled in by libcosmic** (`src/app/mod.rs` `EMBEDDED_FONTS`, unconditional): Open Sans Light, Regular, Semibold, Bold and ExtraBold (SIL OFL 1.1, © 2020 The Open Sans Project Authors, `res/open-sans/LICENSE`), and Noto Sans Mono Regular and Bold (SIL OFL 1.1, `res/noto/LICENSE`). OFL 1.1 requires the copyright notice and license to accompany the font.
> - **Iced-Icons.ttf**, embedded by `iced_graphics` (`iced/graphics/src/text.rs`): it has no license file of its own beside it; iced is MIT. Confirm its origin and license upstream. (FiraSans is behind the `fira-sans` feature, which isn't enabled here.)
> - **The project's own icons** (`data/icons/*-symbolic.svg`): an agent wrote them by hand in the scaffold session (FV-19XWHQ), with no third-party source. List them as the project's own, under the project license.
> - **About 527 crates in the Linux build**, mostly MIT and/or Apache-2.0, plus BSD, ISC, Zlib, Unicode-3.0, MPL-2.0, CC0 and BSL-1.0. MIT, BSD and similar licenses require their copyright notices in redistributed binaries.
>
> Suggested approach: generate `THIRD-PARTY-NOTICES` with a pinned `cargo-about` (or a similar tool), add the fonts' OFL texts, keep a short `docs/BUNDLED-ASSETS.md` (each asset, owner, license, where its notice is), and add the notices to `[package.metadata.deb] assets` under `usr/share/doc/cosmic-ext-nightlight/`. Do it after the license decision in the R33 issue, since the project license appears in the same files.
>
> Labels: Docs

Then:

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The change adds docs, OFL texts, generated notices and .deb assets, all at mode 644. No code in the binary changes. The tests pass (46 passed, 1 ignored), and running the generator again gives exactly the committed THIRD-PARTY-NOTICES, with no local paths or secrets in it. One problem must be fixed: the generator follows symlinks, and its upward search isn't held inside the dependency checkout. A git dependency can therefore get any file the user can read copied into a file that is committed and shipped in the package.
> #1 [must fix] scripts/gen-third-party-notices.py:25: `license_files` reads any entry whose name matches LICEN[CS]E/COPYING/NOTICE, and `os.path.isfile` follows symlinks. Git dependencies are checked out with their symlinks kept: winit's crates already ship `LICENSE -> ../LICENSE` in ~/.cargo/git/checkouts. A git dependency (third-party content, and a git dependency's revision can change on `cargo update`) can therefore ship `LICENSE -> ~/.ssh/id_ed25519`, or any other file the user can read. Its contents would then go verbatim into THIRD-PARTY-NOTICES, which is committed and installed in the published .deb. That file is 19k lines, so a reviewer of the regenerated diff is unlikely to notice. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Resolve each candidate with `os.path.realpath`. Read it only if it is a regular file whose real path lies under the package's root: the crate folder, or for a git dependency the checkout root `~/.cargo/git/checkouts/<repo>/<rev>/`. Otherwise skip it and print a warning to stderr. Also cap the size you read (for example 1 MiB) so a huge or special file can't inflate the output.
> #2 [must fix] scripts/gen-third-party-notices.py:53: The comment says the upward search for a workspace license stays 'inside the checkout', but nothing enforces that. For a crate at `checkouts/<repo>/<rev>/<crate>` with no license file, three steps up reach `~/.cargo/git` and then `~/.cargo`. A deeper layout can reach `$HOME`, and any LICENSE/COPYING/NOTICE file found there would be printed into the shipped notices. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Work out the checkout root (the two path components after `/git/checkouts/`) and stop the loop when `up` would leave it, for example `if not os.path.realpath(up).startswith(checkout_root + os.sep) and up != checkout_root: break`.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding t…

(Shortened here; record.json has all of it.)

> [Event] Your other reviewers' findings, to fix along with the changes asked for:
> #3 [should fix] data/licenses/OFL-NotoSansMono.txt:1: The file is an exact copy of libcosmic's res/noto/LICENSE, and it has no copyright line (no 'Copyright 20xx The Noto Project Authors'). OFL 1.1 requires each copy to carry the copyright notice, and the task asked for that notice. BUNDLED-ASSETS.md names 'The Noto Project Authors' as owner, but the shipped text doesn't say so. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Add the copyright line taken from the Noto Sans Mono font's name table (nameID 0) or upstream notofonts repository to the top of data/licenses/OFL-NotoSansMono.txt, and note its source in BUNDLED-ASSETS.md. Check that OFL-OpenSans.txt has the same line (it does: line 1).
> #4 [should fix] docs/BUNDLED-ASSETS.md:21: The task asked to confirm Iced-Icons.ttf's origin and license upstream. The doc says this was not done (no network), so the row remains an open point and the package ships with an unverified font license. This must be closed before the release, not left to the reader. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Confirm with iced-rs/iced (the font's source and the glyph license, e.g. Fontello's MIT/OFL terms) and replace the 'Open point' text with the confirmed license and source. If it can't be confirmed before merge, record it as an open issue and link it from the doc.
> #5 [should fix] docs/BUNDLED-ASSETS.md:8: The doc says the project is GPL-3.0-only and that `LICENSE` is that license. The installed `LICENSE` file is the GNU AGPL v3 text, and Cargo.toml says GPL-3.0-only. The package would ship a notice that contradicts its own file. The scripts/gen-third-party-notices.py header text repeats the claim and goes into THIRD-PARTY-NOTICES. The license decision (FV-9CWESN) is still open. Until it's made, say the project license is pending, or state the mismatch. Regenerate the notices once the decision lands. (Sonnet 5.5)
> #6 [suggestion] docs/BUNDLED-ASSETS.md:14: The Iced-Icons.ttf origin and license is still an open point, which the task asked to confirm. The engineer had no network, so this is acceptable. Keep a follow-up issue for it. The row also says '510 crates' where the task said about 527. Check the count against the generated file. (Sonnet 5.5)
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept…

(Shortened here; record.json has all of it.)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.toml` | modified | +5 −0 |  |
| `THIRD-PARTY-NOTICES` | added | +19297 −0 |  |
| `data/licenses/OFL-NotoSansMono.txt` | added | +94 −0 |  |
| `data/licenses/OFL-OpenSans.txt` | added | +88 −0 |  |
| `docs/BUNDLED-ASSETS.md` | added | +34 −0 |  |
| `justfile` | modified | +4 −0 |  |
| `scripts/gen-third-party-notices.py` | added | +115 −0 |  |

## Tested

No test runs were reported.

## Summary

Decision landed: GPL-3.0-only. Merge it.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `ed42cb7222` to `265b943062`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
