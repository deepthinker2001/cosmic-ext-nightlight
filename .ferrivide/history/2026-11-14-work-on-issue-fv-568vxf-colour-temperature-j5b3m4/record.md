# Work on issue FV-568VXF: Colour temperature slider

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-568vxf-colour-temperature-slider` · FV-568VXF: Colour temperature slider

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-568VXF: Colour temperature slider
>
> In the popup, add a slider for colour temperature (about 1900 K to 6500 K, default 3500 K) with a label showing the Kelvin value and a short warm/cool hint. Changing it updates the screen live while night light is on, and is saved and restored at startup. Convert Kelvin to gamma ramps in the gamma backend, with unit tests. Still set through the Wayland gamma protocol only (R2).
>
> Labels: Applet

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [should fix] src/gamma.rs:499: New test assertion `assert!(MIN_TEMPERATURE < DEFAULT_TEMPERATURE && DEFAULT_TEMPERATURE < MAX_TEMPERATURE)` has constant operands. `cargo clippy --all-targets` flags it (clippy's 'this assertion has a constant value'), and the change introduces it. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Wrap it in a const block: `const { assert!(MIN_TEMPERATURE < DEFAULT_TEMPERATURE && DEFAULT_TEMPERATURE < MAX_TEMPERATURE) };`, or move the check into a `const _: () = assert!(...);` at module level.
> #2 [suggestion] src/main.rs:334: The `Message::Temperature` handler (busy/retune_pending branching, and the save on every slider step) has no unit test. The pure helpers `retune_needed`, `applied` and `temperature_hint` are tested, but the handler's wiring isn't. The existing `toggle_flips_and_ignores_while_busy` test shows the pattern for testing this kind of state. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Optionally add a test that drives `Nightlight::update` with `Message::Temperature` while busy and enabled, and asserts that `retune_pending` is set and no request is made.
> #3 [should fix] src/main.rs:343: The retune is only queued when `self.enabled` is already true. At startup restore (`busy: saved`, `enabled: false`) and while a toggle-on is in flight, `enabled` is still false. A slider move in that window is saved but not applied, because the in-flight request carries the old Kelvin. The screen then stays at the stale temperature until the next change. (Sonnet 5.5)
>    Suggested fix: Whenever `self.busy` is true, set `retune_pending = true` regardless of `enabled`. `retune_needed(self.enabled, pending)` in `Applied` already checks `enabled` after the result, so it covers that case. Add a test for it.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Understood

Add a colour-temperature slider (1900-6500 K, default 3500) to the popup with Kelvin label and warm/cool hint; live update while on; persist and restore. Kelvin->ramp already exists in gamma.rs; add range constants/clamp and tests.

Plan:

- Add range constants and clamp in gamma.rs with tests
- Worker request carries kelvin
- Settings gains temperature
- Message::Temperature updates, saves, re-applies when enabled, coalescing while busy
- Popup shows slider, label and hint

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `src/gamma.rs` | modified | +34 −0 |  |
| `src/main.rs` | modified | +152 −16 |  |

## Tested

- `cargo test --locked`: 18 passed, 0 failed, 1 skipped

Progress reports:

- Added colour temperature slider (1900–6500 K); builds, 18 tests pass; not tried on a live panel.

## Summary

The popup now has a colour temperature slider from 1900 K to 6500 K, defaulting to 3500 K. `cargo test --locked` passes (18 passed, 1 ignored). I haven't run the applet on a live COSMIC panel, so the slider's look and live effect are untested.

- **Slider and label:** it steps in 100 K. Above it is the label "Colour temperature: N K", with a hint beside it: Very warm, Warm, Neutral warm or Cool.
- **Live updates:** while night light is on, moving the slider retunes the screen. A move that arrives while another request is still in flight is held and applied once, using the latest value. While night light is off, the slider only stores the value.
- **Saved and restored:** the value is saved under the config key `temperature` (in Kelvin) on every change, and read back and clamped to the range at startup. If night light was on when the applet last ran, it comes back at the saved temperature.
- **Gamma backend:** the Kelvin-to-ramp conversion already existed, so I added the 1900 K and 6500 K limits and a clamp function in `src/gamma.rs`. I added unit tests for the clamp, the range ends, and the ramp getting warmer as Kelvin drops. Gamma is still set only through the Wayland gamma protocol (R2).
- **Other changes:** the worker thread now takes a Kelvin value with each request, and the new slider logic is in `src/main.rs`. I also added small tested helpers for the hint text and for deciding when to retune.

The work is committed with "Fixes FV-568VXF".

---

From `968ca811e2` to `0de2d46015`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
