# Work on issue FV-V8VESV: Gamma control backend over Wayland

**Merged** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-v8vesv-gamma-control-backend-over-wayland` · FV-V8VESV: Gamma control backend over Wayland

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-V8VESV: Gamma control backend over Wayland
>
> Write a module that binds zwlr_gamma_control_manager_v1 and gets a gamma control for each wl_output. It builds gamma ramps from a colour temperature (default about 3500 K), applies them on enable, and restores normal colour on disable. Handle outputs being added and removed, and handle the 'failed' event. Keep the control objects alive while night light is on, because the compositor resets gamma when they are destroyed. First check that cosmic-comp advertises the protocol, and report clearly if it does not. Add unit tests for the temperature-to-ramp maths.
>
> Labels: backend

Then:

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The backend contains no unsafe code, no shell or path handling and no secrets. The new dependencies are crates already in Cargo.lock through libcosmic, and the memfd is created with CLOEXEC. Tests pass (6 passed, 1 ignored) and the build is clean. One must-fix: the gamma_size the compositor sends goes straight into a ramp allocation with no upper bound. A misbehaving compositor can make the applet try to allocate about 24 GiB twice and abort.
> - src/gamma.rs:225: `gamma_size` comes from the compositor as a u32, and nothing checks it before it reaches `build_ramp` (line 137). `build_ramp` makes a Vec of `3*size` u16 values, then line 138 copies it into a second Vec of bytes. With size = u32::MAX that is about 24 GiB plus 24 GiB, so the allocation fails and the process aborts (an OOM abort, not an Err). On a 32-bit target `size * 3` can also overflow. Real hardware reports 256 to 4096 entries.
> Fix these and commit; your reviewers then check the fixes.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.lock` | modified | +3 −0 |  |
| `Cargo.toml` | modified | +3 −0 |  |
| `src/gamma.rs` | added | +425 −0 |  |
| `src/main.rs` | modified | +3 −0 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): No must-fix findings from round 1 were required, and this round's only change is the gamma_size bound. A size of 0 or above 65536 is now refused: the control is destroyed and the output marked failed, before any ramp is allocated. The build succeeds and the 6 gamma unit tests pass. The new bound path has no test, but that is a suggestion at most, not a defect.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The only change since the last round is a gamma-size bound: sizes of 0 or above 65536 destroy the control and mark it failed. It's sound, and the tests still pass.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The must-fix is fixed. The GammaSize handler now refuses a size of 0 or above MAX_GAMMA_SIZE (65536) before any allocation, and drops that output's control. The ramp allocation is therefore bounded at about 393 KB per output. Tests pass (6 passed, 1 ignored) and the build is clean.

- [should fix] `src/gamma.rs:232`: The new rejection branch clears `state.outputs[idx].control` without checking that the stored control is the one sending the event. This is the same pattern as my earlier should_fix on the Failed handler. If an old control's event arrives after a newer control replaced it, the live control is dropped without being destroyed.

## Summary

I fixed the must-fix from the cyber review and committed it. The 6 unit tests still pass; I didn't write a test for the new size check, and the live path is unexercised because this compositor doesn't advertise the protocol.

A `gamma_size` from the compositor is now checked before any ramp is built. A size of 0, or one above `MAX_GAMMA_SIZE` (65536, well over the 256–4096 real hardware reports), is refused. The output's control is destroyed and the output is marked failed, the same as a `failed` event, so it isn't retried until the next `enable`. This also closes the 32-bit `3*size` overflow, since the size can no longer get that large.

The change is in `src/gamma.rs`.

---

From `bfcd27531c` to `51084c9b48`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
