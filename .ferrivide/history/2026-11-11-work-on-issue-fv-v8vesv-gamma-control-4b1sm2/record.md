# Work on issue FV-V8VESV: Gamma control backend over Wayland

**In progress** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-v8vesv-gamma-control-backend-over-wayland` · FV-V8VESV: Gamma control backend over Wayland

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

## Summary

The gamma backend is written and committed, but the compositor running here does not advertise `zwlr_gamma_control_manager_v1`. On this session's compositor the backend would return `GammaError::Unsupported`, so night light can't change screen colour there.

I found that with an ignored probe test (`cargo test probe -- --ignored --nocapture`). It connected to the Wayland display and the protocol was missing. I couldn't run `wayland-info` here, so the probe is the only evidence. I assumed this display is cosmic-comp, but I didn't confirm it.

The 6 unit tests for the temperature-to-ramp maths pass. Everything else (the live Wayland paths) is untested, because the protocol isn't available here. I didn't apply any gamma, add or remove an output, or trigger a `failed` event.

How the module (`src/gamma.rs`) works:
- **Setup:** `GammaBackend::new()` returns `GammaError::Unsupported` when the manager isn't advertised. Otherwise it binds the manager and creates one control per output.
- **Ramps:** the colour temperature (default 3500 K, neutral 6500 K) is turned into red, green and blue multipliers using Tanner Helland's blackbody approximation. The ramp is written to a memfd and sent with `set_gamma`.
- **Enable and disable:** `enable(kelvin)` applies the ramp to all outputs, or changes the temperature if already on. `disable()` destroys every control, which restores normal colour.
- **Lifetime:** the control objects stay alive while night light is on, and the backend also disables on drop.
- **Outputs and failures:** an output added later gets a control and the ramp as soon as it reports its gamma size. A removed output has its control destroyed. On `failed` the control is destroyed and not retried until the next `enable`.

The caller has to call `dispatch()` when the Wayland fd is readable so hotplug and `failed` events get processed. `enable`, `disable` and `dispatch` each do one roundtrip.

I added `wayland-client`, `wayland-protocols-wlr` and `rustix` to `Cargo.toml`, and `mod gamma;` plus `#![allow(dead_code)]` to `src/main.rs`. Nothing in the applet's toggle calls the backend yet.

---

From `bfcd27531c` to `16628bf9dc`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
