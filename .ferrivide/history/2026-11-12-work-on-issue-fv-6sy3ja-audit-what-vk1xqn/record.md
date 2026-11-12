# Work on issue FV-6SY3JA: Audit: what the reviews of Engineer 1's work on…

**In progress** 2026-11-12 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-6sy3ja-audit-what-the-reviews-of-engineer-1-s-w` · FV-6SY3JA: Audit: what the reviews of Engineer 1's work on cosmic-ext-nightlight left for later

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

## Summary

(none given)

---

From `615516c496` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
