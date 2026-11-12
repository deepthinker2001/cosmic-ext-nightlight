# Work on issue FV-6SY3JA: Audit: what the reviews of Engineer 1's work on…

**Merged** 2026-11-12 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-6sy3ja-audit-what-the-reviews-of-engineer-1-s-w` · FV-6SY3JA: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-6SY3JA: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later
>
> Findings that didn't hold the merge back (R24):
>
> - [should_fix] src/gamma.rs:1: The task asked the agent to first check that cosmic-comp advertises zwlr_gamma_control_manager_v1 and report clearly. Nothing in the change or the session record reports that check. The code only detects it at runtime (GammaError::Unsupported), so the answer for cosmic-comp is still unknown.
>   - Suggested fix: Check cosmic-comp's source or run a live probe (cargo test -- --ignored --nocapture on a cosmic session) and state the result in the report and the issue.
>   - Raised by Quick reviewer (Haiku 5.5) on Engineer 1's work on cosmic-ext-nightlight
> - [should_fix] src/main.rs:1: #![allow(dead_code)] is a crate-wide suppression added to silence the unused gamma module. It will hide real dead code in the applet later.
>   - Suggested fix: Remove it and mark only the gamma module or its items with #[allow(dead_code)] until the applet calls them.
>   - Raised by Quick reviewer (Haiku 5.5) on Engineer 1's work on cosmic-ext-nightlight
> - [suggestion] src/gamma.rs:402: The ignored live probe test prints the result but asserts nothing, so it can't fail when the compositor lacks the protocol.
>   - Suggested fix: Assert Ok for a known-good compositor, or only print. Keep it as is if the probe is meant to be informational.
>   - Raised by Quick reviewer (Haiku 5.5) on Engineer 1's work on cosmic-ext-nightlight
> - [suggestion] src/main.rs:1: The crate-wide `#![allow(dead_code)]` hides unused-code warnings for the whole crate. Remove it once the applet uses the backend, or scope it to `mod gamma`.
>   - Raised by Sonnet 5.5 on Engineer 1's work on cosmic-ext-nightlight
> - [suggestion] src/gamma.rs:325: `dispatch` uses `roundtrip`, which blocks. The caller must not call it from the UI thread without care. Consider non-blocking `dispatch_pending` plus reading the fd when this is wired into the applet.
>   - Raised by Sonnet 5.5 on Engineer 1's work on cosmic-ext-nightlight
> - [should_fix] src/gamma.rs:228: The Failed handler finds its entry by output name only, then sets `e.control = None` and `failed = true` without checking that the failing proxy is the one stored there. If an older control's `failed` event arrives after a newer control replaced it (for example after a quick enable/disable/enable), the live control is dropped without being destroyed. Night light then stays applied, untracked, after disable.
>   - Suggested fix: Act only when the proxies match: `if state.outputs[idx].control.as_ref() == Some(control) { ... }`. Always call `control.destroy()` on the failing proxy.
>   - Raised by Cyber reviewer (Opus 5.5) on Engineer 1's work on cosmic-ext-nightlight
> - [suggestion] src/gamma.rs:325: `dispatch()` calls `roundtrip`, which blocks until the compositor answers. Drop calls it too, through disable. If the compositor hangs, the applet's UI thread (and its drop) blocks indefinitely. Th…

(Shortened here; record.json has all of it.)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.lock` | modified | +114 −43 |  |
| `Cargo.toml` | modified | +1 −1 |  |
| `data/io.github.cosmic_ext.Nightlight.desktop` | modified | +1 −1 |  |
| `justfile` | modified | +2 −2 |  |
| `src/gamma.rs` | modified | +62 −13 |  |
| `src/main.rs` | modified | +87 −23 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): The change addresses the audit findings: stale-control events are matched against the stored control before clearing it, every failed control is destroyed, dispatch is non-blocking (dispatch_pending plus prepare_read; the wayland-backend docs confirm reads return WouldBlock rather than block), Drop no longer round-trips, dead_code is scoped to mod gamma, libcosmic is pinned to a rev, and the justfile uses --locked. cargo test --locked passes: 9 passed, 1 ignored (the informational live probe). I did not run the applet itself or verify the cosmic-comp protocol check or the Categories change against a live session.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The change covers the audit findings. I ran `cargo test --locked`: 9 tests pass and the live probe is ignored. I did not run the applet on a COSMIC session. The stale-control event checks, the scoped dead_code allow, non-blocking dispatch, the pinned libcosmic rev, `--locked` in the justfile and the new unit tests are all there. No must-fix issues.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The fixes for the security findings hold up. A gamma control's events now change an output's state only when that control is the one stored for the output. A stale control is always destroyed, both on Failed and on a bad gamma size, so a live control can't be orphaned. The untrusted gamma_size bound is still in place. libcosmic is pinned by rev to the commit the lockfile already used, so no upstream code changed, and the justfile builds with --locked. Drop no longer makes a blocking round trip, and idle dispatch doesn't block (prepare_read/read). cargo test --locked gives 9 passed and 1 ignored, and clippy --locked --all-targets is clean. I found nothing that must be fixed.

## Summary

I fixed every should_fix across the three issues and most of the suggestions, and committed. The 9 unit tests pass (plus 1 ignored live probe) and clippy is clean. I never ran the applet against a live compositor, so none of the Wayland behaviour is exercised.

**Should-fix items**
- **Stale gamma control:** the `Failed` and bad-gamma-size handlers in `src/gamma.rs` now clear an output's entry only when the event comes from the control stored for it. The failing control is always destroyed.
- **Partial enable failure:** if enabling fails partway, `src/main.rs` now calls `disable` before returning the error, so no output stays warm while the icon shows off.
- **State tests:** the toggle and reply logic is now two plain functions, `toggle_request` and `applied`, with 3 unit tests. They cover toggle-while-busy, success saving to config, and failure keeping the real state.
- **libcosmic pin:** `Cargo.toml` now has `rev = 8fa46cc…`, the commit the lockfile already used.
- **Desktop Categories:** `Categories=COSMIC;` is now `Utility;`. I didn't run `desktop-file-validate`, and I kept `COSMIC` in `Keywords`.

**Suggestions done**
- **Dead-code allow:** `#[allow(dead_code)]` is now only on `mod gamma`.
- **`--locked`:** added to the justfile build and clippy targets.
- **Non-blocking dispatch:** `dispatch()` no longer round-trips; it flushes, reads what's available and dispatches. `enable` and `disable` still do a blocking round-trip so their result is known, which is safe because they only run on the worker thread. `Drop` now destroys the controls and flushes without waiting on the compositor.
- **Idle worker:** it now drains compositor events every 500 ms, and starts a fresh connection after a broken one.
- **Failed-restore choice:** a failed startup restore keeps the saved flag set, so the next start retries. This is explained in a comment on `applied`.
- **Quit behaviour:** a comment records that the process exit closes the Wayland connection, which makes the compositor reset gamma, even though the detached worker thread's backend is never dropped.
- **Live probe:** its doc comment now says it is informational and asserts nothing.

**Not done or unverified**
- **cosmic-comp protocol check:** I couldn't confirm that it advertises `zwlr_gamma_control_manager_v1`. The code still detects this only at runtime, as `GammaError::Unsupported`. Someone needs to run `cargo test -- --ignored --nocapture probe_compositor` on a COSMIC session.
- **Stale "on" icon:** the worker now reads events, but it doesn't tell the UI when the compositor drops every gamma control. The icon can still show "on" in that case.
- **Desktop entry:** whether the panel still finds the applet through `X-CosmicApplet` is unconfirmed on a COSMIC session.

---

From `615516c496` to `99d9935211`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
