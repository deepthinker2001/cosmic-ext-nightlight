# Work on issue FV-FK5E4X: Automatic schedule

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-fk5e4x-automatic-schedule` · FV-FK5E4X: Automatic schedule

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
- `cargo test --locked`: 27 passed, 0 failed, 1 skipped

Progress reports:

- Added automatic schedule (Off/Custom times) with spin-button pickers; tests pass, committed.
- Fixed #1 (retry after failed scheduled switch, with test); kept toggler for #2.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Finding #1 is fixed: `Applied` now calls `schedule_last_after_reply`, which forgets a failed request for the window state, so the next tick retries the switch. The new unit test covers the failure, the retry and the success and other-state cases. `cargo test --locked` passes with 27 passed and 1 ignored. Finding #2 was a suggestion, so it doesn't hold the merge.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Finding #1 is fixed. A failed request for the state the schedule asked for now clears `schedule_last`, so the next tick retries, and a new unit test covers the retry. Tests pass (27 passed). Finding #2 was a suggestion, and keeping the toggler is fine.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): Finding #1 is fixed. `schedule_last_after_reply` clears the remembered window state when a scheduled request fails, so the next check tries again, and a new unit test covers this. If the backend keeps failing, the retry runs once per 20-second tick, not in a tight loop. The new lines raise no security concerns. Finding #2 was a suggestion, so it doesn't block the merge. `cargo test --locked` passes: 27 passed, 1 ignored.

## Summary

I fixed finding #1 and committed it. `cargo test --locked` passes (27 passed, 1 ignored). I haven't run it on a live panel.

- **#1 (fixed):** a failed request for the state the schedule asked for now clears the remembered window state. The next 20-second check then applies it again, instead of waiting for the window's next edge. A failed manual toggle for the opposite state leaves the memory alone, so it doesn't trigger a schedule override.
- **How it works:** the rule is a small pure function, `schedule_last_after_reply`, which `Message::Applied` calls. A new test, `failed_scheduled_switch_is_retried`, covers the retry.
- **Retry rate:** if the backend keeps failing, the retry repeats every 20 seconds until it works.
- **#2 (kept as is):** I left the "Schedule: custom times" toggler. It gives the same two choices, Off or Custom times, with the times shown only when it's on. I didn't decline it, since it doesn't fit any of the decline reasons. Say if you want a radio or dropdown instead.

---

From `1f1dd02e27` to `036e1ca398`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
