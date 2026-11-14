# Work on issue FV-TXZZS6: Applet is slow: left click does nothing, popup toggle…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `fv-txzzs6-applet-is-slow-left-click-does-nothing-p` · FV-TXZZS6: Applet is slow: left click does nothing, popup toggle unclickable, slider and toggles lag

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

Then:

> [Event] Cyber reviewer (Opus 5.5) asks for changes: The change adds no unsafe code, dependencies, process spawning, file paths taken from input, or secrets. The new saver thread and the latest-wins request state are sound, and a hostile caller can't make either grow without bound, because each kind of setting keeps only its latest value. X-HostWaylandDisplay hands the applet the host compositor's display. That is needed for gamma control and grants nothing an unsandboxed user process couldn't already reach, so I don't count it as a widening. One regression: a failed save now logs the user's custom location to stderr, and from there it reaches the journal. fmt is clean, clippy has no warnings, and 52 tests pass (1 ignored).
> #1 [must fix] src/saver.rs:97: The error message prints the whole Save with {save:?}. For Save::Location(Some((lat, lon))) that writes the user's exact coordinates to stderr, which cosmic-panel sends to the system journal, where other tools and users in the journal groups can read it. The old code logged only "cannot save location: {e}", without the value. This is a new exposure of private data. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Log the kind of setting, never its value. For example, add `fn kind(&self) -> &'static str` on Save that returns "state", "temperature", "schedule", "auto setting" or "location", and write `eprintln!("nightlight: cannot save {}: {e}", save.kind());`. If you keep the Debug derive for tests, make sure nothing logs it.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> [Event] Your other reviewers' findings, to fix along with the changes asked for:
> #2 [suggestion] src/saver.rs:1: The saver is a detached thread that writes after 250 ms of quiet. A setting changed within 250 ms of the applet exiting (panel restart, logout) can be lost, because process exit doesn't wait for the thread. Optionally flush on shutdown, or keep the JoinHandle and drop the sender at exit. (Sonnet 5.5)
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

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
| `CHANGELOG.md` | modified | +7 −0 | Unreleased entries for the fixes |
| `data/io.github.cosmic_ext.Nightlight.desktop` | modified | +1 −0 | X-HostWaylandDisplay=true so cosmic-panel keeps WAYLAND_DISPLAY for the gamma worker's own connection |
| `src/main.rs` | modified | +87 −202 | Use Requests and Saver; Worker::request takes a Target; Applied carries only the result; icon toggles on press; UI shows the wanted state |
| `src/requests.rs` | added | +188 −0 | New pure latest-wins request state (wanted/in flight/applied) with unit tests, replacing busy, retune_pending, toggle_request, applied and Retune |
| `src/saver.rs` | added | +219 −0 | Save::kind for log messages instead of Debug (#1); Drop closes the channel and joins so pending writes flush (#2); spawn_with takes the Config opener so tests use a temp folder; two new tests |

### Decisions

- **Add X-HostWaylandDisplay=true to the desktop entry instead of changing how gamma.rs connects**: cosmic-panel (wrapper_space.rs) drops WAYLAND_DISPLAY without it, and the X_PRIVILEGED_WAYLAND_SOCKET it offers is a single fd already taken by libcosmic's token handler (applet-token feature), so it can't be shared. Turned down: Reuse X_PRIVILEGED_WAYLAND_SOCKET (already owned by libcosmic); Connect before libcosmic starts and steal WAYLAND_SOCKET (would break the applet's panel surface).
- **Latest-wins coalescing with one request in flight, instead of dropping input while busy**: A dropped toggle reads as 'can't be clicked'. Coalescing keeps slider retunes at one round trip per reply, never a backlog. Turned down: A queue of every request (lags behind a drag); Debouncing gamma updates on a timer (adds latency to the display change).
- **Save settings on a background thread after 250 ms of quiet, rather than on slider release only**: Covers every write path (slider, schedule, auto, location typing, replies) with one mechanism, and keeps the UI thread free of fsync. Turned down: Slider on_release saving only (leaves the other writes on the UI thread).
- **Kept the existing rule: save the enabled flag only on success**: An earlier session decided a failed startup restore must keep the saved flag so the next start retries.
- **Flush on drop by closing the channel and joining the thread**: The thread already writes at once when the channel disconnects, so joining is all that's needed. The cost at exit is at most one write (about 10-30 ms). Turned down: Catching SIGTERM to flush (a new dependency and signal handling, for a 250 ms window).

## Tested

- `cargo test`: 52 passed, 0 failed, 1 skipped
- `cargo test`: 54 passed, 0 failed, 1 skipped

Progress reports:

- FV-TXZZS6 committed (1cf1c4928d): applet can reach gamma control (X-HostWaylandDisplay), settings saved off the UI thread (22–36 ms → ~0.13 µs per slider step), toggles/slider never dropped while busy, left click on press. 52 tests pass; not verified on a live panel.
- Review fixes committed (4287aa69e1): save errors name only the setting (no location in the journal); dropping the Saver flushes pending writes and joins the thread. fmt clean, clippy clean, 54 tests pass.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Both earlier findings are fixed. The error log now prints only the setting's kind. Dropping the Saver closes the channel and joins the thread, so pending writes are flushed. I didn't build or run the tests this round. Both changes have tests.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The coordinates leak is fixed: the error message now prints only the setting's kind, and a test covers it. The saver also flushes on drop and waits for its thread, with a test. All 54 tests pass.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): Finding #1 is fixed. src/saver.rs now logs only the setting's kind through Save::kind(), and a test checks that the location's value stays out of it. The saver now keeps its thread handle and, on drop, closes the channel and waits for pending writes; that adds no security issue. fmt is clean, clippy has no warnings, and 54 tests pass (1 ignored).

## Summary

I've fixed both review findings and committed them as `4287aa69e1` (Fixes FV-TXZZS6). Formatting and clippy are clean and all 54 tests pass (1 live probe is skipped).

- **#1 (must fix), location leaking into the log:** when a save failed, the error message printed the whole setting, including a custom location's coordinates. The applet's error output ends up in the system journal. The message now names only the setting ("cannot save location: …"), and a new test checks that it contains no coordinates.
- **#2 (suggestion), settings lost on exit:** a change made in the 250 ms before the applet exits is now saved instead of lost. When the saver is dropped it writes what's waiting at once and the applet waits for that write, at most one write (about 10–30 ms). A new test checks this against a temporary folder, not the user's real settings.

Two limits remain:
- **Killed applet:** the flush only happens on a clean shutdown. If the panel kills the applet without one, a change from its last 250 ms can still be lost. Catching that would mean adding signal handling; I judged that too much for such a short window.
- **Live panel:** the earlier fixes (the `X-HostWaylandDisplay` key, background saving, keeping clicks made while a change is applied, toggling on press) still haven't been tried on a live panel.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `f4a92c2c25` to `4287aa69e1`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
