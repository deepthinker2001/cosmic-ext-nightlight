# Work on issue FV-V8VESV: Gamma control backend over Wayland

**In progress** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-v8vesv-gamma-control-backend-over-wayland` · FV-V8VESV: Gamma control backend over Wayland

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-V8VESV: Gamma control backend over Wayland
>
> Write a module that binds zwlr_gamma_control_manager_v1 and gets a gamma control for each wl_output. It builds gamma ramps from a colour temperature (default about 3500 K), applies them on enable, and restores normal colour on disable. Handle outputs being added and removed, and handle the 'failed' event. Keep the control objects alive while night light is on, because the compositor resets gamma when they are destroyed. First check that cosmic-comp advertises the protocol, and report clearly if it does not. Add unit tests for the temperature-to-ramp maths.
>
> Labels: backend

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.lock` | modified | +3 −0 |  |
| `Cargo.toml` | modified | +3 −0 |  |
| `src/gamma.rs` | added | +412 −0 |  |
| `src/main.rs` | modified | +3 −0 |  |

## Tested

No test runs were reported.

## Summary

(none given)

---

From `bfcd27531c` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
