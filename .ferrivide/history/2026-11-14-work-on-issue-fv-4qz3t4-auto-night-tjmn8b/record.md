# Work on issue FV-4QZ3T4: Auto night light: smooth sunset/sunrise ramp

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-4qz3t4-auto-night-light-smooth-sunset-sunrise-r` · FV-4QZ3T4: Auto night light: smooth sunset/sunrise ramp

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

Then:

> [Ferrivide] A scan of your change to Rust files found 2 smells the Rust standards (the rust-development skill) cover. Fix what you agree is a problem, or say in your closing report why it stays; this note comes once for each.
> - src/main.rs:78 [loop-allocation] `handle_request` is called inside a loop in `spawn`, and does `Vec::new` through `GammaBackend::new` (src/gamma.rs:324) on every call (fix: RS4. Hot paths allocate nothing; see RS4)
> - src/main.rs:615 [wide-struct] `Nightlight` has 17 fields, over the limit of 15: group the ones that change together into structs of their own (fix: RS7. Keep files and impls a reasonable size; see RS7)

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [should fix] src/main.rs:108: The RS4 finding is moved, not resolved. The worker loop still calls handle_request (line 78), which calls GammaBackend::new (through gamma.rs:324's Vec::new) whenever the backend is absent. The report says new 'is no longer called in the worker's loop', which isn't accurate. The allocation runs once per (re)connect, not per frame, so it isn't a hot path. Either hoist or avoid it, or state that justification in a code comment and in the report so the reviewer can accept it under R24. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Add a comment at handle_request saying the connect runs only when the backend is None (once per connection, not a hot path), and correct the report wording. Alternatively avoid the Vec::new in gamma.rs:324 with Vec::with_capacity or by reusing the outputs Vec. Then rerun the smell scan.
> #2 [suggestion] src/main.rs:724: manual_kelvin maps ramp 0% (the whole day) to the full target, but 1% maps to a ramp value near neutral. A manual switch-on during the day therefore goes to full warmth, and the next step (1%, at sunset minus 30 min) drops it to about 6470 K. That may be intended ('holds until the next step'), but nothing says so. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Add a comment saying why 0 means the target here, or make it consistent with auto_action (neutral at 0%) and say what a manual press does in the README.
> #3 [suggestion] src/main.rs:1055: The latitude and longitude boxes appear only when auto is on or no location exists. A user with a timezone location and auto off can't enter a custom location until they first switch auto on. The task says the popup lets the user override the location. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Show the boxes whenever a location is in use, or always, so the override is reachable with auto off.
> #4 [suggestion] src/main.rs:883: update() is now about 230 lines, and the settings popup closure added about 60 lines inside it. RS7 asks for a function under about 100 lines when touched; the popup view could move into its own function. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Move the ToggleSettings popup body (the content closure) into a fn popup_content(state: &Nightlight) -> Element, and the auto handlers into a helper. This can be a separate change if the engineer prefers.
> #5 [suggestion] src/solar.rs:27: `sun_times` returns minutes since local midnight with no wrap. At longitudes far from their zone's meridian, sunrise or sunset can fall below 0 or above 1440, and the ramp for that event is then misplaced for today. This is an edge case; consider clamping or computing the neighbouring days. (Sonnet 5.5)
> #6 [suggestion] src/location.rs:51: `text[1.min(text.len())..]` slices by bytes. If the coordinates column of a zone table starts with a non-ASCII character, the slice falls inside the character and the applet panics at start-up (`timezone_locati…

(Shortened here; record.json has all of it.)

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
| `README.md` | modified | +31 −0 | documents Auto |
| `src/location.rs` | added | +163 −0 | new: timezone coordinates and typed-degree parsing, tests |
| `src/main.rs` | modified | +394 −70 | auto state, messages, popup UI, config, tick; handle_request helper takes backend creation out of the worker loop (RS4) |
| `src/solar.rs` | added | +240 −0 | new: solar times, ease, ramp percent, step logic, tests |

### Decisions

- **Percent 0 disables gamma instead of applying 6500 K**: Neutral is exactly off; avoids a needless gamma control. Turned down: apply 6500 K.
- **Reuse the 20 s tick rather than a per-step timer**: Steps are ~36 s apart and the tick also covers resume from sleep. Turned down: dedicated timer per step.

## Tested

- `cargo test --locked`: 45 passed, 0 failed, 1 skipped

Progress reports:

- Auto sunrise/sunset ramp added and committed; cargo test --locked passes (45 passed, 1 ignored). Not run on a live panel.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): All six findings from the last round are addressed in 98dc6f5. The RS4 justification is now in a comment at handle_request, with the report corrected. The manual 0% behaviour is documented in code and README. The latitude and longitude boxes always show. The popup moved into popup_content. Ramp events wrap into the day. ISO 6709 parsing slices after the ASCII sign, with the non-ASCII case added to the test. `cargo test --locked` passes (46 passed, 1 ignored), and the build has no warnings.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): All six earlier findings are addressed in the lines changed since my last review. `cargo test --locked` passes (46 passed, 1 ignored). The RS4 allocation now has an R24 justification comment. The popup is its own function. The latitude and longitude boxes always show. The README explains the manual toggle at 0%. Wrapped sunrise and sunset times are handled and tested. The location parser can no longer panic on non-ASCII text.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): My finding #6 is fixed. `parse_iso6709` now removes a leading ASCII `+` or `-` before it looks for the split, so a non-ASCII first character returns None instead of panicking. A test with `é+3114+12128` covers it. The lines changed since my last review add nothing that concerns security: the popup code moved into its own function, the location boxes now always show and still pass through the same checks, and the ramp wrap is plain arithmetic on finite values. `cargo test --locked`: 46 passed, 1 ignored. The build is clean.

## Summary

I fixed all six findings and committed the fixes. `cargo test --locked` passes (46 passed, 1 ignored). I haven't run it on a live panel.

1. **RS4 on `handle_request`:** I fixed the wording and kept the code. My earlier report was wrong to say `new` is no longer called in the worker's loop. It is called through `handle_request`, but only while the backend is `None`, so once per connection or reconnect. The `Vec::new` inside it doesn't allocate until an output is pushed. A comment on `handle_request` now says this, so the smell scanner will probably still flag that line.
2. **Manual switch-on in auto mode:** a comment in `manual_kelvin` and a README sentence now say what happens. A manual switch-on uses the current ramp temperature, or your full target when the ramp is at 0% (daytime). The next 1% step then takes over.
3. **Location boxes:** the latitude and longitude boxes now always show in the popup, so the override works with Auto off. The "Use timezone location" button still appears only once a custom location is set.
4. **`update()` size:** the popup body is now its own function, `popup_content`, outside `update`.
5. **Sun events outside 0–1440 minutes:** a sunrise before midnight or a sunset after it is now handled. `ramp_percent` shifts the sunrise into the day and looks at the one daily cycle that starts half a window before it. A new test, `events_outside_the_day_wrap`, covers both cases.
6. **Zone-table parsing panic:** `parse_iso6709` now checks the leading sign before splitting, so non-ASCII text can't make it panic. `"é+3114+12128"` is in the rejected-input test.

---

From `ce6cac9711` to `98dc6f5444`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
