# Work on issue FV-9CWESN: License: the build contains GPL code, and the…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-9cwesn-license-the-build-contains-gpl-code-and` · FV-9CWESN: License: the build contains GPL code, and the project's own license is inconsistent (R33)

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

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): The change does what the user decided: LICENSE is now the full GPL-3.0 text (674 lines, matching the canonical length, with no AGPL clauses left apart from section 13's mention of AGPL, which is part of the GPL text itself). Cargo.toml's license stays GPL-3.0-only, the deb metadata gains a license-file entry in cargo-deb's format, the README License section states GPL-3.0-only with the reason, and the CHANGELOG gets a line. `cargo build --locked` succeeds and `cargo fmt --all -- --check` passes. No code changed, so the test suite and clippy aren't affected by this diff.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): LICENSE is now the standard GPL-3.0 text (the Affero mentions left are GPLv3 section 13). Cargo.toml, the README and the CHANGELOG say GPL-3.0-only, and the deb metadata gets license-file. No Rust code changed. I didn't run the build or tests.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change only touches license text and metadata: LICENSE, a `license-file` entry in Cargo.toml's deb metadata, the README and the CHANGELOG. It has no code, dependencies, permissions or secrets, so there's nothing to fix for security. LICENSE is byte-for-byte the same as the system's /usr/share/common-licenses/GPL-3. No AGPL text is left outside the GPL's own section 13. `cargo test --locked` passes: 46 passed, 1 ignored.

## Summary

I replaced `LICENSE` with the GPL-3.0 text, so the project is now GPL-3.0-only everywhere. I didn't build the .deb. The only check was that `cargo metadata --locked` still parses `Cargo.toml`.

- **`LICENSE`:** it now holds the GPL-3.0 text, copied from `/usr/share/common-licenses/GPL-3`. It was the AGPL-3.0 text before.
- **`Cargo.toml`:** `license` was already `GPL-3.0-only`. I added `license-file = ["LICENSE", "0"]` under `[package.metadata.deb]`, so the .deb ships the license text. I haven't checked how cargo-deb treats this next to the existing `license` field.
- **`README.md`:** the License section now says GPL-3.0-only, links `LICENSE`, and says why: libcosmic's applet support links GPL-3.0-only cosmic-panel crates.
- **`CHANGELOG.md`:** I added a line under Unreleased → Changed. The blank line I left around it splits the list slightly oddly, which I didn't tidy.

I found no other license text in headers or packaging files.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.

---

From `a6feeb03eb` to `21d9dcbd25`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
