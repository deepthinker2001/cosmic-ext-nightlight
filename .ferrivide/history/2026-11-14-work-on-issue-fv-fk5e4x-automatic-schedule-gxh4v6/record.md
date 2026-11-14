# Work on issue FV-FK5E4X: Automatic schedule

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-fk5e4x-automatic-schedule` · FV-FK5E4X: Automatic schedule

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-FK5E4X: Automatic schedule
>
> In the popup, add a schedule: Off (manual only, the default) or Custom times, with start and end time pickers. When a schedule is on, night light turns on at the start time and off at the end time, including across midnight, and after resume from sleep or restart. Manual toggling still works and holds until the next schedule change. Saved with the other settings. Unit-test the time logic. Don't add location/sunset lookup in this issue.
>
> Labels: Applet

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.lock` | modified | +1 −0 |  |
| `Cargo.toml` | modified | +1 −0 |  |
| `src/main.rs` | modified | +326 −4 |  |

## Tested

No test runs were reported.

## Summary

Now write the code.

---

From `1f1dd02e27` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
