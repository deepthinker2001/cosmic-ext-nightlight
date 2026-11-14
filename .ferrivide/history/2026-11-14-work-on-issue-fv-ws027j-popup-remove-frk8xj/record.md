# Work on issue FV-WS027J: Popup: remove the manual schedule, add a "use…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-ws027j-popup-remove-the-manual-schedule-add-a-u` · FV-WS027J: Popup: remove the manual schedule, add a "use timezone" toggle that replaces the lat/long entry

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-WS027J: Popup: remove the manual schedule, add a "use timezone" toggle that replaces the lat/long entry
>
> User request:
> - Remove the manual schedule toggle and its start/end pickers entirely. The user only wanted the automatic (sunset/sunrise) night light schedule. Remove the code, config keys use and tests that become dead (schedule.rs etc.); keep old saved config values harmless (ignore them, no crash).
> - Add a "From time zone" toggle in the popup. When on (the default), the location comes from the system time zone (location.rs, zone.tab/zone1970.tab) and the latitude/longitude entry is hidden/replaced by it, showing which zone/place was used. When off, show the latitude/longitude entry (location_custom). Persist the toggle in the config; migrate sensibly from the existing location_custom key (location_custom true -> toggle off). If the time zone can't be resolved, fall back to the lat/long entry and say so.
> - Keep auto mode working as now (auto_on, ramp, slider behaviour). Update README.md (settings section) and CHANGELOG.md (Unreleased) to match, and any knowledge-relevant docs. Tests: unit tests for the config migration and the location choice.
>
> Do this after the performance issue (FV-perf) merges, since both change popup.rs and app.rs.
>
> Labels: Applet
>
> The performance fix is now merged: gamma requests go through src/requests.rs, settings saves go through src/saver.rs (never write config on the UI thread, never log setting values). Build on that. Commit early and in small steps.

## Understood

Drop the manual custom-times schedule (toggle, start/end pickers, its logic, tests and config key use; old saved schedule_* values are simply ignored). Add a "From time zone" toggle (default on) to the popup: on, the auto-mode location comes from the system time zone and the popup shows the zone and coordinates instead of the latitude/longitude entry; off, the entry is shown. Persist the toggle in a new config key, migrating from location_custom (true -> toggle off). If the zone can't be resolved, show the entry and say so. Auto mode, ramp and slider behave as now. Update README settings and CHANGELOG Unreleased; unit tests for migration and location choice.

Assumptions:

- The issue names popup.rs/app.rs/schedule.rs, but all this code lives in src/main.rs (the module split was discarded, knowledge k16); I change src/main.rs, src/location.rs and src/saver.rs.
- New key location_from_timezone (bool). When it's missing, the toggle reads as !location_custom (missing -> on). location_custom is no longer written.
- Saved latitude/longitude are loaded whenever valid, even with the toggle on, so switching the toggle off brings back the typed values.
- The place shown is the zone name (e.g. Europe/Paris) with its coordinates. The time zone is still read once at start-up, as before.
- If the toggle goes off and no coordinates have been typed, auto switches off, as it already does today when there's no location.

Plan:

- location.rs: return the zone name with its coordinates, and add a pure Setting type plus a choose() function that picks the timezone, the typed coordinates or the fallback.
- main.rs: remove the schedule code, messages, fields, keys and tests; Settings load with migration; popup toggle and conditional entry; the tick runs only while auto is on.
- saver.rs: remove Save::Schedule; Save::Location carries the Setting.
- Tests: migration (old keys, new key wins, old schedule keys ignored) and location choice.
- README and CHANGELOG; fmt, clippy, tests; commit.

Out of scope:

- Re-reading the time zone while the applet runs
- Deleting old schedule keys from users' config files (they're ignored instead)
- The module split of main.rs

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `CHANGELOG.md` | modified | +8 −0 |  |
| `README.md` | modified | +11 −16 |  |
| `src/location.rs` | modified | +141 −4 |  |
| `src/main.rs` | modified | +228 −457 |  |
| `src/saver.rs` | modified | +20 −20 |  |

## Tested

No test runs were reported.

## Summary

Only past release notes mention the schedule, and those stay as history. Final fmt, clippy and tests, then the docs commit:

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `a970acc40c` to `ffa6566fc8`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
