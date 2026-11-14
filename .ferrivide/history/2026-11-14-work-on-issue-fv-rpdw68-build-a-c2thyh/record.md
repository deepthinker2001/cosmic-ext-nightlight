# Work on issue FV-RPDW68: Build a .deb package

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-rpdw68-build-a-deb-package` · FV-RPDW68: Build a .deb package

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-RPDW68: Build a .deb package
>
> Add .deb packaging (cargo-deb or a justfile target) installing the applet binary, desktop file and icon. Pure build config, no code changes. Dependencies must be right for COSMIC. Document how to build it locally.
>
> Labels: Release

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [suggestion] Cargo.toml:15: `section = "x11"` marks the package as an X11 application, which is wrong for a Wayland COSMIC applet. Package managers and software centres use the section to file it. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Use `section = "utils"` (or "gnome", the closest existing section for desktop integration).
> #2 [suggestion] Cargo.toml:23: `depends` names cosmic-comp, a hard dependency that pulls in the whole compositor. The applet only needs the gamma protocol, which this compositor provides; the choice is reasonable but should be a deliberate one. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Keep it if the applet is meant to run only under cosmic-comp. Otherwise consider `Recommends` for cosmic-comp, and confirm the choice in the README.
> #3 [suggestion] Cargo.toml:24: The `$auto` libc6 floor (>= 2.39) comes from the build host's glibc, so the package only installs on equally new or newer systems. Say so in the README, or build on the oldest target release. (Sonnet 5.5)
> #4 [suggestion] README.md:48: `cargo install cargo-deb` resolves the tool's dependencies fresh and unpinned, so the packaging tool's supply chain can drift from build to build. The project already uses `--locked` for its own builds. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Use `cargo install --locked cargo-deb` (optionally pin `--version 3.8.0`, the version this was checked with).
> #5 [suggestion] Cargo.toml:20: The comment says `$auto` adds libwayland-client. The built package's Depends is `cosmic-comp, cosmic-panel, libc6 (>= 2.39), libxkbcommon0 (>= 0.5.0)`: there's no libwayland-client because wayland-client 0.31 uses the pure-Rust backend. This isn't a security risk, but the comment misleads anyone auditing the dependencies. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Change the comment to say `$auto` adds whichever shared libraries the binary actually links (libc6 and libxkbcommon0 today).
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.toml` | modified | +19 −0 | [package.metadata.deb] with assets and dependencies |
| `README.md` | modified | +17 −0 | how to build and install the .deb |
| `justfile` | modified | +3 −0 | deb recipe |

## Tested

- 0 passed, 0 failed

Progress reports:

- Added cargo-deb packaging (Cargo.toml metadata, `just deb`, README section); built a real .deb and checked its contents and dependencies.

## Summary

The `.deb` packaging is added and committed. I ran `cargo deb --locked`, which built the package, and I inspected its contents and dependency line. I haven't installed it or run it on a COSMIC session.

- **`Cargo.toml`:** a new `[package.metadata.deb]` section lists the same four files `just install` copies:
  - the binary in `/usr/bin`
  - the desktop file in `/usr/share/applications`
  - the on and off icons in `/usr/share/icons/hicolor/scalable/status`
- **`justfile`:** a new `just deb` recipe runs `cargo deb --locked`.
- **`README.md`:** a new "Build a .deb package" section covers `cargo install cargo-deb`, `just deb` and `sudo apt install ./target/debian/cosmic-ext-nightlight_*.deb`.

**Dependencies:** the built package depends on `cosmic-comp`, `cosmic-panel`, `libc6 (>= 2.39)` and `libxkbcommon0`.
- **Package names:** I wrote `cosmic-panel` and `cosmic-comp` from memory and didn't check them against a COSMIC apt repo.
- **`libc6` floor:** `cargo-deb` took it from the machine I built on. To support older distros, build on the oldest one you target.

I made no code changes.

---

From `806260f1f1` to `a017781ffa`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
