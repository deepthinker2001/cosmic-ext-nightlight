# Work on issue FV-RPDW68: Build a .deb package

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-rpdw68-build-a-deb-package` · FV-RPDW68: Build a .deb package

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

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): All five findings from the last round are addressed in ae5f092: the section is now `utils`, cosmic-comp is a Recommends, the libc6 floor is documented in the README, cargo-deb is installed with `--locked` (version noted), and the Cargo.toml comment now matches what $auto actually links. I rebuilt the package with `cargo deb --locked`; it produces Depends: cosmic-panel, libc6 (>= 2.39), libxkbcommon0 (>= 0.5.0), Recommends: cosmic-comp, Section: utils. Nothing must be fixed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): All five earlier findings are fixed: section is now utils, cosmic-comp is a Recommends, the libc6 floor is documented, cargo-deb is installed with --locked, and the $auto comment is corrected.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): All five findings from the last round are fixed: the section is now utils, cosmic-comp moved to Recommends and the README says so, the libc6 minimum is documented, the README installs cargo-deb with --locked (checked with 3.8.0), and the $auto comment is accurate. I rebuilt the .deb: its Depends and Recommends match the comment, it still has no maintainer scripts, and the tests pass (9 passed, 1 ignored). No new issues in the changed lines.

## Summary

I fixed all five review findings and committed. I rebuilt the `.deb` and checked its metadata with `dpkg-deb`, but haven't installed it.

1. **Section:** `section` is now `utils` instead of `x11`.
2. **`cosmic-comp`:** it moves from `Depends` to `Recommends`, so it no longer pulls in the whole compositor. The README says the toggle won't work without it. The built package now shows `Depends: cosmic-panel, libc6 (>= 2.39), libxkbcommon0 (>= 0.5.0)` and `Recommends: cosmic-comp`.
3. **libc6 floor:** the README now says the minimum comes from the build machine, so build on the oldest release you want to support.
4. **`cargo-deb` install:** the README uses `cargo install --locked cargo-deb` and mentions version 3.8.0.
5. **Comment:** the `Cargo.toml` comment now says `$auto` adds the libraries the binary actually links (libc6 and libxkbcommon0), and that wayland-client uses the pure-Rust backend.

---

From `806260f1f1` to `ae5f092d1c`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
