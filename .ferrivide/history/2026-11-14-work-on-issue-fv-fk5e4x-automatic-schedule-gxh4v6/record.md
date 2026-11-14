# Work on issue FV-FK5E4X: Automatic schedule

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-fk5e4x-automatic-schedule` · FV-FK5E4X: Automatic schedule

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-FK5E4X: Automatic schedule
>
> In the popup, add a schedule: Off (manual only, the default) or Custom times, with start and end time pickers. When a schedule is on, night light turns on at the start time and off at the end time, including across midnight, and after resume from sleep or restart. Manual toggling still works and holds until the next schedule change. Saved with the other settings. Unit-test the time logic. Don't add location/sunset lookup in this issue.
>
> Labels: Applet

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [should fix] src/main.rs:544: `check_schedule` records the window state in `schedule_last` when it starts the request, before the result is known. In `Message::Applied` (around line 749) a failed request just clears `busy`, so a failed scheduled switch (say, at 21:00) is not retried until the window's next edge, possibly hours later. Night light then stays in the wrong state for that whole window. (Quick reviewer (Haiku 5.5))
>    Suggested fix: On `Applied` with an error for a scheduled request, reset `self.schedule_last = None` so the next tick reapplies the window state (or store the want that was requested and clear `schedule_last` only on failure). Add a unit test for `schedule_step` or the failure path showing the retry.
> #2 [suggestion] src/main.rs:726: The popup uses a toggler labelled "Schedule: custom times", while the task described an Off or Custom times choice. The behaviour matches, but if the user wants a two-option control, a radio or dropdown would fit the wording better. Keep the toggler if that's acceptable. (Quick reviewer (Haiku 5.5))
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.lock` | modified | +1 −0 |  |
| `Cargo.toml` | modified | +1 −0 |  |
| `src/main.rs` | modified | +354 −4 |  |

## Tested

- `cargo test --locked`: 26 passed, 0 failed, 1 skipped

Progress reports:

- Added automatic schedule (Off/Custom times) with spin-button pickers; tests pass, committed.

## Summary

The popup now has an automatic schedule: Off, the default, or Custom times with start and end pickers. It builds and `cargo test --locked` passes (26 passed, 1 ignored). I haven't run it on a live panel or in a real session.

- **Popup:** a "Schedule: custom times" toggle. When it's on, Start and End rows appear. Each time is an hour spin button, a colon and a minute spin button. Minutes step by 5; hours step by 1. Defaults are 21:00 to 07:00.
- **Saved settings:** `schedule_on`, `schedule_start` and `schedule_end` are saved with the other settings. The times are minutes since midnight.
- **Turning on and off:** night light is on from the start time up to, but not including, the end time. The window can cross midnight. Equal start and end times mean no window.
- **Checks:** the schedule is checked every 20 seconds. It's also checked at start-up, when you switch the schedule on, and when you change either time.
- **Resume from sleep:** the next check applies the current window state. If the machine slept through a whole window, that window is missed.
- **Manual toggling:** the schedule only acts when the window changes, so a manual toggle holds until the next start or end. If the window changes while a backend request is in flight, the check waits and retries 20 seconds later.
- **Start-up:** with a schedule on, the clock decides the state at launch instead of the saved on/off choice.
- **Tests:** the time logic is pure, in `src/main.rs`. Tests cover same-day windows, across-midnight windows, equal times, first-check versus change-only behaviour, and the schedule being off.

I added `chrono` (with only the `clock` feature) to read local time. It was already in `Cargo.lock`, so `--locked` still builds. As asked, there's no location or sunset lookup.

I chose spin buttons because libcosmic has no ready-made time picker, and a dropdown would open a nested popup inside the applet popup, which I didn't want to risk.

The code is committed with "Fixes FV-FK5E4X".

---

From `1f1dd02e27` to `c1fc62528a`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
