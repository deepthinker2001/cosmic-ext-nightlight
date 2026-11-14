# Work on issue FV-4QZ3T4: Auto night light: smooth sunset/sunrise ramp

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-4qz3t4-auto-night-light-smooth-sunset-sunrise-r` · FV-4QZ3T4: Auto night light: smooth sunset/sunrise ramp

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-4QZ3T4: Auto night light: smooth sunset/sunrise ramp
>
> Add an "Auto (sunrise/sunset)" toggle to the settings popup. Do after FV-568VXF (temperature slider) and FV-FK5E4X (schedule); it builds on both.
>
> Location: the DEFAULT is the system timezone's coordinates (from /usr/share/zoneinfo/zone1970.tab or zone.tab, read locally, no network). The popup also lets the user override it by entering latitude and longitude; once set, those win, with a way to go back to the timezone default. If neither works, show a note in the popup and leave auto off. No IP lookup, no external services.
>
> Behaviour: compute sunrise and sunset for today from the location (standard solar-position formula; handle polar day/night). Read the user's temperature setting from the slider (the target). Ramp in 1% steps: percent 0 is neutral (6500 K) and 100 is the user's target. Use a sinusoidal ease (sin^2 of the progress, or an equivalent half-cosine curve) over a ramp window before and after each event: evening ramps 0 to 100% around sunset, morning ramps 100 to 0% around sunrise. Make the window length a constant (default 60 min, centred on the event) and say so in the popup. Apply a new step only when the whole percent changes, via a timer, and recompute after resume from sleep. If the user changes the target temperature, the ramp uses the new value at once. The manual toggle still works and holds until the next step. Auto and the custom schedule are mutually exclusive: turning one on turns the other off.
>
> Persist settings with the others. Unit-test the solar times, the ramp curve and the 1% step logic. Gamma still goes only through the Wayland protocol (R2). Document it in the README (FV-J959H4 covers docs: add this to its scope).
>
> Labels: Applet
>
> Also fix the smell scan finding RS4 at src/main.rs:66: GammaBackend::new is called inside a loop in spawn and does Vec::new (src/gamma.rs:324) each call; hoist or avoid the allocation.

## Understood

Add an Auto (sunrise/sunset) mode to the settings popup. Location defaults to the system timezone's coordinates from zone.tab or zone1970.tab, and the user can override it with latitude and longitude, with a way back to the default. Sun times come from a solar formula and handle polar day and night. A 1% sin^2 ramp over a 60 min window centred on each event moves the gamma toward the slider's target. A step is applied when the whole percent changes, from the existing 20 s tick, which also catches up after sleep. Auto and the custom schedule are mutually exclusive. Settings persist and the README documents the mode. Also fix the RS4 smell at the GammaBackend::new call in the worker loop.

Assumptions:

- Percent 0 means gamma disabled (neutral)
- Manual toggle in auto mode uses the current ramp temperature

Plan:

- Add pure modules src/solar.rs and src/location.rs, with unit tests
- Wire auto state, messages, popup and config keys into main.rs
- Move backend creation out of the worker loop body into a helper
- Update the README and run cargo test

Out of scope:

- Live panel testing
- IP geolocation

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `README.md` | modified | +29 −0 |  |
| `src/location.rs` | added | +161 −0 |  |
| `src/main.rs` | modified | +342 −32 |  |
| `src/solar.rs` | added | +216 −0 |  |

## Tested

No test runs were reported.

## Summary

Now Settings.

---

From `ce6cac9711` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
