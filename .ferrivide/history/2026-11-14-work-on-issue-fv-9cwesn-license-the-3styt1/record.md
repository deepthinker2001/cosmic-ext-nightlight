# Work on issue FV-9CWESN: License: the build contains GPL code, and the…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-9cwesn-license-the-build-contains-gpl-code-and` · FV-9CWESN: License: the build contains GPL code, and the project's own license is inconsistent (R33)

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-9CWESN: License: the build contains GPL code, and the project's own license is inconsistent (R33)
>
> Found by the release pass. The release requirement R33, as the release job states it, says no build contains GPL, LGPL or AGPL code. R33 isn't among the project's recorded requirements (only R0–R2 are). Today every build breaks it in three ways, and each fix needs the user's decision:
>
> 1. **The project's own license is GPL-family, and stated two ways.** `Cargo.toml` (`license = "GPL-3.0-only"`), the README's License section and the .deb metadata say GPL-3.0-only. `LICENSE` (from the GitHub initial commit 5554517) is the **GNU Affero GPL v3** text. No session recorded a license decision. Pick one license, then make `LICENSE`, `Cargo.toml` and the README agree.
> 2. **GPL-3.0-only code is linked in through libcosmic.** libcosmic's `applet` feature (needed for R1) pulls `cosmic-panel-config` and `xdg-shell-wrapper-config` from pop-os/cosmic-panel @03c1358. The crates have no license field; their repository's `LICENSE.md` and workspace `Cargo.toml` say GPL-3.0-only. Check with `cargo tree -i cosmic-panel-config`. Under a strict R33 a COSMIC panel applet built with libcosmic can't be released at all. Options: accept these crates as an exception to R33, ask upstream to relicense or split them, or reword R33 (for example, "no copyleft beyond what libcosmic's applet support requires").
> 3. Fine as they are, for the record: `configparser` (MIT OR LGPL-3.0-or-later) and `self_cell` (Apache-2.0 OR GPL-2.0-only) are dual-licensed and can be used under MIT or Apache-2.0. libcosmic, cosmic-theme, cosmic-config and cosmic-settings-daemon-zbus are MPL-2.0 (weak copyleft, not GPL).
>
> Done when: the user has chosen the project license and how R33 treats the cosmic-panel crates, and LICENSE, Cargo.toml and the README match that choice.
>
> Labels: Docs
>
> User decision: relicense to GPL-3.0-only, the license compatible with libcosmic's applet feature (which links GPL-3.0-only cosmic-panel-config / xdg-shell-wrapper-config). Replace the AGPL-3.0 LICENSE with the GPL-3.0 text, make Cargo.toml, the deb metadata, README and any headers or packaging files agree, and add a CHANGELOG line.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `CHANGELOG.md` | modified | +2 −0 |  |
| `Cargo.toml` | modified | +1 −0 |  |
| `LICENSE` | modified | +81 −68 |  |
| `README.md` | modified | +1 −1 |  |

## Tested

No test runs were reported.

## Summary

Deb metadata: check Cargo.toml for a deb section.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `a6feeb03eb` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
