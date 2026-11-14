# Work on issue FV-TXZZS6: Applet is slow: left click does nothing, popup toggle…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-txzzs6-applet-is-slow-left-click-does-nothing-p` · FV-TXZZS6: Applet is slow: left click does nothing, popup toggle unclickable, slider and toggles lag

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-TXZZS6: Applet is slow: left click does nothing, popup toggle unclickable, slider and toggles lag
>
> User report from a live COSMIC panel (current main, 0.2.x):
> 1. The applet runs really slow.
> 2. Left click on the panel icon does nothing (it should toggle night light).
> 3. In the right-click popup: the night light toggle can't be clicked; the temperature slider is very sluggish; the schedule and auto toggles take a noticeable delay before they change.
>
> Find the root causes in the code (src/app.rs, popup.rs, worker.rs, config.rs, auto.rs, location.rs, solar.rs, schedule.rs) and fix them. Likely suspects to check, not conclusions: work done on the UI thread in update() or view() (reading/parsing zone.tab or zone1970.tab, config reads/writes, chrono/local time, solar calculation) on every message or every frame; a config write on every slider move; the 20 s tick or a subscription doing heavy work or re-created each update; the worker round trip blocking the UI or retune_pending never clearing so toggles are ignored; the mouse_area wrapped around the icon button swallowing left clicks; the popup's toggle being disabled while a request is "in flight" that never ends.
>
> Requirements: left click toggles; the popup toggle works at once; slider moves feel immediate (write config/gamma at most once per short interval or on release, not on every event; keep the display change responsive); toggles show their new state at once. Keep the unit-test suite passing and add unit tests for any pure logic you change. You can't see a live panel from your sandbox: reason from the code, say plainly what you could and couldn't verify, and state each cause you found and its fix in your report. Another issue (FV-next: remove manual schedule, add timezone location toggle) will change popup.rs after yours merges, so keep your change focused.
>
> Labels: Applet
>
> Notes from the first attempt (it ran out of budget before committing, so start fresh): the real code is a monolithic src/main.rs (~1190 lines) plus gamma.rs, location.rs, solar.rs; the module names in the issue are stale. Confirmed suspects: Config::new/set done synchronously on the UI thread per slider event and per reply; toggle_request drops input while the worker is busy. Also check gamma.rs: the worker's dispatch/prepare_read/guard.read after an idle 500 ms might block waiting for events and strand later requests. Work in small steps, commit early, and fix the causes first; split out only what you touch (small module for pure input/debounce logic). FV-WS027J edits main.rs right after.

## Understood

Fix FV-TXZZS6: on a live COSMIC panel the applet feels slow, a left click on the icon does nothing, the popup's night-light toggle can't be clicked, the slider lags and the schedule/auto toggles change late. Find the root causes in src/main.rs and gamma.rs and fix them, with unit tests for any pure logic changed.

Assumptions:

- cosmic-comp exposes zwlr_gamma_control to clients on the main WAYLAND_DISPLAY socket (I can't verify that here).
- Losing a setting change made within 250 ms of the applet being killed is acceptable.
- Showing the wanted state at once, then reverting if the backend fails, is what 'toggles show their new state at once' means.

Plan:

- Desktop file: add X-HostWaylandDisplay=true. cosmic-panel (wrapper_space.rs) strips WAYLAND_DISPLAY from an applet's environment unless the desktop entry has that key, and libcosmic consumes WAYLAND_SOCKET, so the gamma worker's Connection::connect_to_env() fails and every request errors: the icon and popup toggle snap back to off (the 'does nothing' symptoms).
- New pure module src/requests.rs: latest-wins request state (wanted / in flight / applied) replacing busy, retune_pending, toggle_request, applied() and Retune. Toggles and the slider are never dropped while a request is in flight; the UI shows the wanted state at once; a failure shows the real state again without a retry loop.
- New src/saver.rs: one long-lived thread that owns a single Config and writes settings after a 250 ms quiet period, keeping only the latest value per kind. This takes the Config::new and fsync-heavy Config::set calls off the UI thread (they currently run per slider step and per reply).
- Icon button: on_press_down instead of on_press, as libcosmic's own applet buttons do, so the toggle fires on the press and doesn't depend on the release reaching the button.
- Measure Config::set cost on this disk before/after; run fmt, clippy and the tests; commit.

Out of scope:

- Splitting the rest of main.rs (FV-WS027J edits it next).
- Popup layout and the schedule UI (FV-next changes it).
- Per-retune ramp allocations in gamma.rs (a few kB per output, not the cause).

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `CHANGELOG.md` | modified | +7 −0 |  |
| `data/io.github.cosmic_ext.Nightlight.desktop` | modified | +1 −0 |  |
| `src/main.rs` | modified | +87 −202 |  |
| `src/requests.rs` | added | +188 −0 |  |
| `src/saver.rs` | added | +153 −0 |  |

## Tested

No test runs were reported.

## Summary

Checks pass: fmt is clean, clippy has no warnings, 52 tests pass (1 live probe ignored), and the smell scan finds nothing. Committing.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `f4a92c2c25` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
