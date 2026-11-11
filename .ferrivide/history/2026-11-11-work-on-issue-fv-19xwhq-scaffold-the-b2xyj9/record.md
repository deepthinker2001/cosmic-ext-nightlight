# Work on issue FV-19XWHQ: Scaffold the libcosmic panel applet

**Merged** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-19xwhq-scaffold-the-libcosmic-panel-applet` · FV-19XWHQ: Scaffold the libcosmic panel applet

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-19XWHQ: Scaffold the libcosmic panel applet
>
> Create the Rust crate for a COSMIC panel applet using libcosmic's applet support. Add a panel icon button that shows an on or off state, a .desktop entry with X-CosmicApplet so the panel can find it, symbolic icons for both states, and a justfile with build and install targets. For now, the toggle only flips state in the UI.
>
> Labels: applet

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.gitignore` | modified | +1 −0 |  |
| `Cargo.lock` | added | +6851 −0 |  |
| `Cargo.toml` | added | +12 −0 |  |
| `data/icons/io.github.cosmic_ext.Nightlight-off-symbolic.svg` | added | +1 −0 |  |
| `data/icons/io.github.cosmic_ext.Nightlight-on-symbolic.svg` | added | +1 −0 |  |
| `data/io.github.cosmic_ext.Nightlight.desktop` | added | +14 −0 |  |
| `justfile` | added | +26 −0 |  |
| `src/main.rs` | added | +56 −0 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): The applet does what the task asked: a libcosmic applet with an icon button that flips an on/off state in the UI, a desktop entry with X-CosmicApplet=true, symbolic icons for both states, and a justfile with build and install targets. `cargo build --release` succeeds and `cargo clippy --all-targets` reports no warnings. `just --dry-run install` lists the expected install paths. No tests exist, which the task didn't ask for; the toggle is trivial.

- [should fix] `data/io.github.cosmic_ext.Nightlight.desktop:5`: desktop-file-validate reports an error for Categories=COSMIC; because COSMIC is not a registered main category, so the entry may land in a catch-all menu section.
- [suggestion] `src/main.rs:1`: No tests cover the toggle state logic. It is trivial now, but a small unit test on Message::Toggle flipping `enabled` would guard it once it grows.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Scaffold matches the task: the crate compiles (cargo check passes), with an icon button that flips on/off state in the UI, a desktop entry with X-CosmicApplet, both symbolic icons, and a justfile with build and install targets. I didn't run it in a panel.

- [suggestion] `Cargo.toml`: libcosmic is an unpinned git dependency. Cargo.lock does pin it, so builds are reproducible. Consider adding a rev or branch.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): A small UI-only scaffold: no input parsing, no file, network or process access, no unsafe code, no secrets. The justfile's install and uninstall use fixed paths with 0755/0644 modes, and `cargo build --locked` and `cargo test --locked` both pass (0 tests). The only concern is supply chain: libcosmic is a git dependency with no rev in Cargo.toml. Cargo.lock does pin it to a commit, so this isn't a blocker.

- [should fix] `Cargo.toml:9`: libcosmic is a git dependency that follows the default branch with no rev. Cargo.lock pins it to commit 8fa46cc. But a `cargo update`, or a build anywhere without the lockfile, silently pulls whatever upstream HEAD is, along with its 50+ transitive git dependencies.
- [suggestion] `justfile:11`: `cargo build --release` re-resolves the lockfile whenever Cargo.toml and Cargo.lock drift, so the git dependencies can change without anyone noticing.

## Summary

The applet crate is scaffolded and committed. `cargo check` passes against current libcosmic. I did not run the applet itself or `just build`/`just install`, and I only dry-ran `just install`, so the icons and the panel entry haven't been seen working.

- **Toggle:** the panel icon button flips an on/off flag in the UI only. It switches between the two icons and does nothing else yet.
- **Desktop entry:** `data/io.github.cosmic_ext.Nightlight.desktop` has `X-CosmicApplet=true`.
- **Icons:** `data/icons/` has symbolic on and off SVGs. They're simple placeholder moon shapes, which you may want to redraw.
- **Justfile:** targets are `build`, `check`, `clean`, `install` and `uninstall`. `install` copies the binary, desktop entry and icons under `/usr`.
- **Dependency:** libcosmic is a git dependency on the default branch, so builds follow its newest commit. The older checkout in the local cargo cache (`430f3d4`) fails to resolve its own dependencies, so I didn't pin to it. I also added `target` to `.gitignore`.

Next time, pin libcosmic to a known-good commit once you want reproducible builds. `Cargo.lock` is committed and holds it for now.

---

From `3241c2de3d` to `5eff73e5b9`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
