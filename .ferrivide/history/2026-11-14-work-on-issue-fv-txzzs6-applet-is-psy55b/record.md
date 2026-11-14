# Work on issue FV-TXZZS6: Applet is slow: left click does nothing, popup toggle…

**Discarded** 2026-11-14 · Engineer 2 (project agent) · `chatgpt:gpt-6.1-sol` · branch `fv-txzzs6-applet-is-slow-left-click-does-nothing-p` · FV-TXZZS6: Applet is slow: left click does nothing, popup toggle unclickable, slider and toggles lag

0 tokens in, 0 out

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

Then:

> [Message from Project manager] Agreed: work in the real src/main.rs. My file names were from stale notes. Fix the causes first. Split out only what you touch and need (RS7), such as a small module for the pure input/debounce logic, and don't reshuffle the rest: FV-WS027J (remove manual schedule, add timezone toggle) edits the same file right after. Report each cause found and its fix.
>
> [Issue FV-WS027J, as it stands now]
> FV-WS027J: Popup: remove the manual schedule, add a "use timezone" toggle that replaces the lat/long entry
> Status: Backlog
> Labels: Applet
> Origin: made in Ferrivide by Project manager
>
> User request:
> - Remove the manual schedule toggle and its start/end pickers entirely. The user only wanted the automatic (sunset/sunrise) night light schedule. Remove the code, config keys use and tests that become dead (schedule.rs etc.); keep old saved config values harmless (ignore them, no crash).
> - Add a "From time zone" toggle in the popup. When on (the default), the location comes from the system time zone (location.rs, zone.tab/zone1970.tab) and the latitude/longitude entry is hidden/replaced by it, showing which zone/place was used. When off, show the latitude/longitude entry (location_custom). Persist the toggle in the config; migrate sensibly from the existing location_custom key (location_custom true -> toggle off). If the time zone can't be resolved, fall back to the lat/long entry and say so.
> - Keep auto mode working as now (auto_on, ramp, slider behaviour). Update README.md (settings section) and CHANGELOG.md (Unreleased) to match, and any knowledge-relevant docs. Tests: unit tests for the config migration and the location choice.
>
> Do this after the performance issue (FV-perf) merges, since both change popup.rs and app.rs.

> [Ferrivide] Your helper's answer to "Inspect pinned libcosmic/iced widget event code (dependencies accessible via cargo cache) to determine whether mouse_area around applet.icon_button swallows left click; inspect toggler hit handling and provide focused fix if any. Task FV-TXZZS6 actual UI is src/main.rs view and popup_content. Also e":
>
> - The working view in `src/main.rs:1182–1190` puts `self.core.applet.icon_button(icon).on_press(Message::Toggle)` inside `mouse_area(button)`, adding only `.on_right_press(Message::ToggleSettings)`. The button’s left-click handler is still present, and the mouse area has no left-press handler here; this code alone gives no indication that it should swallow left clicks.
> - `popup_content` builds the settings UI and maps its messages to `cosmic::Action::App` (`src/main.rs:990`); it is not part of the applet icon’s hit handling.
> - `Nightlight` uses `cosmic::SingleThreadExecutor` (`src/main.rs:993–997`). I can’t establish further executor implications from the application code alone.
> - I couldn’t inspect the pinned libcosmic/iced widget event implementation: the cargo checkout is outside the allowed readable paths. So I can’t confirm event propagation or determine whether the wrapper intercepts the click downstream, nor provide a dependency-grounded fix. No code changes made.

> [Ferrivide] Your helper's answer to "Investigate gamma.rs dispatch prepare_read guard.read: does pinned wayland-client backend block if no events? Worker calls it after idle 500ms; could strand all requests? Read dependency implementation and recommend minimal fix with evidence. Also inspect config Config::new/set for costly I/O (fsync":
>
> I couldn’t inspect the code: the project tools exposed empty directories for both supplied worktrees, and searches returned no results. So I can’t verify `gamma.rs`, the pinned `wayland-client` implementation, `Config::new`/`set`, or cite file:line evidence, and can’t give an evidence-based fix or measurement recommendation.

## Implemented

No files changed.

## Tested

- `cargo test --locked`: 47 passed, 0 failed, 1 skipped

Progress reports:

- Baseline suite passes (47 passed, 1 ignored); tracing synchronous config persistence and busy-gated input before editing.

## Summary

I’ll trace the input handling and retune path, check the earlier design decisions, then make focused fixes and run the tests and required checks. I can’t verify live panel behavior here, so I’ll distinguish code-level findings from what still needs a panel check.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `b64f63baf0` to uncommitted work. The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
