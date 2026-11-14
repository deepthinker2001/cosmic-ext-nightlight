# Work on issue FV-568VXF: Colour temperature slider

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-568vxf-colour-temperature-slider` · FV-568VXF: Colour temperature slider

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-568VXF: Colour temperature slider
>
> In the popup, add a slider for colour temperature (about 1900 K to 6500 K, default 3500 K) with a label showing the Kelvin value and a short warm/cool hint. Changing it updates the screen live while night light is on, and is saved and restored at startup. Convert Kelvin to gamma ramps in the gamma backend, with unit tests. Still set through the Wayland gamma protocol only (R2).
>
> Labels: Applet

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
| `src/main.rs` | modified | +122 −16 |  |

## Tested

No test runs were reported.

## Summary

(none given)

---

From `968ca811e2` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
