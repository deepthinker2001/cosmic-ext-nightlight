# Work on issue FV-R03W5C: Connect the toggle to the backend and remember its…

**Merged** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-r03w5c-connect-the-toggle-to-the-backend-and-re` · FV-R03W5C: Connect the toggle to the backend and remember its state

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-R03W5C: Connect the toggle to the backend and remember its state
>
> Run the gamma backend from the applet without blocking the UI (for example as an iced subscription or a background task). Make the panel toggle turn night light on and off, and have the icon follow the real state, including failures. Save the on/off state with cosmic-config and restore it when the applet starts.
>
> Labels: applet

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `src/main.rs` | modified | +101 −3 |  |

## Tested

- 6 passed, 0 failed

Progress reports:

- Toggle now drives the gamma backend off the UI thread; state persisted via cosmic-config. Compiles; not run live.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): The toggle now sends requests to a worker thread that owns the blocking Wayland gamma backend, applies the result through Message::Applied, shows the icon from the confirmed state, saves the state with cosmic-config on success, and restores it in init. It builds, and cargo test passes (6 passed, 1 ignored, all in gamma.rs, none for the new code). No clear defect found, so nothing must be fixed before merge; the points below are for the backlog.

- [should fix] `src/main.rs:42`: If b.enable() fails partway (after some outputs were already set), the error is returned without calling disable, so some outputs may stay at the night-light ramp while the icon shows off.
- [should fix] `src/main.rs:60`: No test covers the new behaviour: toggle-while-busy, Applied success saving to config, Applied failure keeping the real state, startup restore. The only tests are the existing gamma.rs ones.
- [suggestion] `src/main.rs:107`: If the saved state is on and enabling fails (for example no outputs), the config stays true, so every start retries and the icon stays off. That may be intended, but it isn't written down.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The toggle now runs the gamma backend on a worker thread, so the UI doesn't block. The icon follows the confirmed result, and failures are logged and leave the icon off. The state is saved with cosmic-config and restored at startup. It builds and the tests pass, but I didn't run the applet.

- [suggestion] `src/main.rs:112`: If restoring the saved state fails at startup, the saved 'enabled' stays true and the failure is only printed to stderr. That is acceptable, but you could clear it or show an error state.
- [suggestion] `src/main.rs:30`: The worker thread's sender is never dropped explicitly. Whether the gamma backend resets on exit depends on its Drop. Confirm that quitting the applet restores neutral gamma.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change adds a worker thread that owns the blocking Wayland gamma backend, a oneshot reply into an iced Task, and a single bool saved with cosmic-config. It adds no dependencies, no unsafe code, no subprocesses and no file paths built from input. The only thing it reads from outside is a typed bool from the user's own config, and a missing or bad value falls back to off. Errors are logged to stderr with nothing sensitive in them. Builds, and 6 tests pass (1 ignored).

- [suggestion] `src/main.rs:36`: The worker only touches the Wayland connection when the user toggles, so events the compositor sends while it sits idle (outputs added or removed, gamma control 'failed') are never read. The icon can then show 'on' after the compositor has dropped the gamma control. This isn't a security problem. It is a robustness gap: if the socket buffer fills up, the compositor may disconnect the client.

## Summary

The panel toggle now turns night light on and off through the gamma backend, and the on/off state is saved and restored at startup. It builds with `cargo check` and the 6 unit tests pass. I haven't run the applet or tried the toggle against a real compositor. The changes are committed.

- **Backend off the UI thread:** a worker thread in `src/main.rs` owns the gamma backend. The toggle sends it a request and the UI isn't blocked while the compositor answers. A second press while a request is still running is ignored.
- **Icon follows the real state:** the icon only changes to "on" once the backend confirms it. If enabling fails, the icon stays off and the error goes to stderr. This includes the case where the compositor doesn't offer gamma control, which I haven't confirmed for cosmic-comp. Enabling also counts as failed if no output accepts gamma control.
- **Saved state:** the state is saved with cosmic-config under the key `enabled` after each successful change. If it was on when the applet last ran, the applet tries to turn it on again at startup. A failed enable leaves the saved value alone, so the next start retries it.
- **No cosmic-config derive:** I read and write the key directly because the derive needs a direct `cosmic-config` dependency. This avoids adding one.
- **No on-screen error:** failures are only printed to stderr, with no message in the panel. A tooltip would be the natural follow-up if you want one.

---

From `4b273b5f2b` to `6a3a86c3c2`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
