# Work on issue FV-WS027J: Popup: remove the manual schedule, add a "use…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-ws027j-popup-remove-the-manual-schedule-add-a-u` · FV-WS027J: Popup: remove the manual schedule, add a "use timezone" toggle that replaces the lat/long entry

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
| `CHANGELOG.md` | modified | +8 −0 | Unreleased notes the schedule removal and the new toggle. |
| `README.md` | modified | +11 −16 | The settings section drops the schedule and describes the From time zone toggle and its fallback. |
| `src/location.rs` | modified | +141 −4 | timezone_location now returns the zone name with its coordinates. Added Setting, Source, choose(), place_name() and their tests. |
| `src/main.rs` | modified | +228 −457 | Removed the schedule's types, logic, messages, state, popup rows and tests. Added Settings.location with the migration (from_timezone_setting), the From time zone toggle, the conditional lat/long entry, location_note, and tests for the migration and the notes. |
| `src/saver.rs` | modified | +20 −20 | Removed Save::Schedule. Save::Location now carries location::Setting. Tests updated. |

### Decisions

- **New config key location_from_timezone; while it's missing it is read as !location_custom (neither saved: on). location_custom is no longer written.**: Users who typed coordinates keep them in use, everyone else gets the time zone, and the old key can't override a later choice. Turned down: Reuse location_custom with an inverted meaning (it would mix old and new semantics); Bump CONFIG_VERSION (that would discard every saved setting).
- **Typed latitude/longitude are loaded and kept while the toggle is on.**: Switching the toggle off brings back what was typed. This replaces the old 'Use timezone location' button, which cleared the coordinates.
- **Old schedule_* keys are left in the config file and never read.**: Harmless, and nothing to fail at start-up. A test checks they're ignored.
- **The pure location choice is location::choose, which returns a Source enum (Timezone, Custom, TimezoneUnresolved). The popup text comes from the pure location_note.**: Both are unit-testable without a panel (RS3, RS5).
- **The code changes are in src/main.rs, not popup.rs, app.rs or schedule.rs.**: Those files don't exist: the module split was discarded (knowledge k16).

## Tested

- `cargo test`: 56 passed, 0 failed, 1 skipped

Progress reports:

- FV-WS027J done: the custom-times schedule is removed and a "From time zone" toggle (default on) chooses the auto location, carrying over the old location_custom setting. fmt and clippy are clean; 56 tests pass, 1 ignored. Committed as ffa6566fc8 and 001e402c62.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The change removes the manual schedule and adds the "From time zone" toggle. It also adds migration from location_custom, ignores old schedule keys, and updates the README and CHANGELOG. Format check is clean and 56 tests pass. I didn't run clippy or read the diff line by line.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The change does what the task asked. The schedule code and tests are removed, and old schedule keys are ignored (there is a test). The "From time zone" toggle migrates from location_custom, falls back to the lat/long entry with a note, and shows the zone used. Saves still go through saver.rs, and no setting values are logged. fmt and clippy are clean, 56 tests pass, and both commit messages follow C4.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change adds no new way in for hostile input. The zone name from /etc/localtime, /etc/timezone or TZ is only compared with zone.tab entries and shown in the popup. Saved coordinates are range-checked (location::valid) on load, and the toggle falls back to a safe default. Settings are still saved through the saver thread, and the logs name the setting, not its value. The change has no unsafe code, no new dependencies and no command execution. fmt is clean, clippy gives no warnings, and 56 tests pass (1 ignored).

## Summary

I've removed the custom-times schedule and added a "From time zone" switch to the popup. Formatting and clippy are clean, and all 56 tests pass (1 ignored). It hasn't been tried on a live COSMIC panel; the README already says that for the popup.

**Schedule removed:** the popup no longer has the custom-times switch or its start/end pickers. Its code, messages, settings saving and tests are gone, and the 20-second check now runs only while Auto is on. Old saved `schedule_on`/`schedule_start`/`schedule_end` values stay in the config file but are never read; a test checks they're ignored.

**"From time zone" switch (on by default):**
- **On:** Auto's location comes from the system time zone. The popup shows the zone and place used, e.g. `New York (America/New_York): 40.71, -74.01`, and hides the latitude/longitude boxes.
- **Off:** the latitude/longitude boxes are shown and the typed values are used.
- **Time zone location not found:** the popup says so and shows the boxes.
- Typed coordinates are kept while the switch is on, so turning it off brings them back. This replaces the old "Use timezone location" button.
- If you turn the switch off and no coordinates have been typed yet, Auto switches itself off. That's how it already worked when there was no location; turn Auto back on after typing them.

**Saved settings:** the switch is saved under a new key, `location_from_timezone`. Until that key exists, an old `location_custom = true` reads as the switch off. Otherwise it starts on, and the old key no longer matters once the new one has been saved.

Auto mode, the ramp and the slider work as before. Settings are still saved through `src/saver.rs`, off the UI thread, and no setting values are logged.

**Where it changed:**
- **Code:** the issue mentions `popup.rs` and `app.rs`, but that split was never merged, so the changes are in `src/main.rs`, `src/location.rs` and `src/saver.rs`.
- **Docs:** README's settings section and CHANGELOG's Unreleased section are updated.
- **Tests:** new ones cover the settings migration, old schedule keys being ignored, the location choice, the popup's location text and place names.

I've sent the project manager a note updating the knowledge entry on Auto mode and removing the outdated schedule entry. Feature F2's title still says "colour temperature and schedule" and may need retitling. The work is committed in two commits, `ffa6566fc8` (code) and `001e402c62` (docs), and both end with "Fixes FV-WS027J".

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `a970acc40c` to `001e402c62`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
