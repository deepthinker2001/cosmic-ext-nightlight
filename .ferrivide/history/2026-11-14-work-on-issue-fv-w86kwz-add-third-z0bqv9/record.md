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

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.toml` | modified | +5 −0 |  |
| `THIRD-PARTY-NOTICES` | added | +19297 −0 |  |
| `data/licenses/OFL-NotoSansMono.txt` | added | +92 −0 |  |
| `data/licenses/OFL-OpenSans.txt` | added | +88 −0 |  |
| `docs/BUNDLED-ASSETS.md` | added | +34 −0 |  |
| `justfile` | modified | +4 −0 |  |
| `scripts/gen-third-party-notices.py` | added | +85 −0 |  |

## Tested

No test runs were reported.

## Summary

Fake the binary to check the asset list.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `a6feeb03eb` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
