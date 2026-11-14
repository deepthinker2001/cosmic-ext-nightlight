# Work on issue FV-7C81CR: Settings popup on right click

**Merged** 2026-11-14 · Engineer 2 (project agent) · `claude-sonnet-5-5` · branch `fv-7c81cr-settings-popup-on-right-click` · FV-7C81CR: Settings popup on right click

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-7C81CR: Settings popup on right click
>
> Right-clicking the panel icon opens a settings popup (left click still toggles). If libcosmic's applet popup can't take right click, find the closest working way and say so. Add a persisted settings struct in cosmic_config next to 'enabled', with a version bump if needed. Popup shows the on/off switch for now. Keep gamma at the current fixed ~3500 K until the next issue.
>
> Labels: Applet

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [should fix] src/main.rs:239: Still open from round 1: the popup open/close and PopupClosed id handling has no tests. Extract the id transition into a pure helper and unit-test it: closing the matching id clears it, a different id leaves it, and closing with none open is a no-op. (Quick reviewer (Haiku 5.5))
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `src/main.rs` | modified | +98 −6 | Settings struct (cosmic_config), mouse_area right-click opens/closes popup via app_popup, popup shows toggler |

### Decisions

- **Wrap the icon button in mouse_area with on_right_press**: libcosmic's applet button only has left-press hooks; the mouse area gives right click and the popup still uses libcosmic's app_popup. Turned down: cosmic_config_derive CosmicConfigEntry: needs serde and macro paths this crate lacks; hand-written load/save per key instead; version bump: not needed, 'enabled' key unchanged and new fields default when missing.

## Tested

- 9 passed, 0 failed, 1 skipped
- 12 passed, 0 failed, 1 skipped

Progress reports:

- Right-click settings popup with on/off switch added; builds and unit tests pass, not run on a live panel.
- Fixed finding #1: popup_after_close helper with 3 unit tests; 12 pass.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Finding #1 is fixed: the popup id transition is now the pure helper `popup_after_close`, and three unit tests cover the matching id, a different id, and no popup open. `PopupClosed` uses the helper, so the tested logic is the logic that runs. The build is clean and `cargo test` passes (12 passed, 1 ignored live probe). No open findings remain.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The popup id transition is now a pure helper, `popup_after_close`, with three unit tests that cover the cases I asked for. Tests pass.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The quick reviewer's finding #1 is fixed: the id handling for a closed popup is now a pure helper, popup_after_close, with three unit tests that match the finding (the matching id clears it, a different id keeps it, none open stays none). The PopupClosed handler behaves as before. The new lines raise no security concerns, and cargo test gives 12 passed and 1 ignored.

## Summary

I fixed finding #1 and committed it. The popup close handling now lives in a pure helper, `popup_after_close`, in `src/main.rs`, and `PopupClosed` calls it.

Three new unit tests cover it:
- Closing the matching id clears the popup.
- Closing a different id leaves the popup open.
- Closing with no popup open changes nothing.

`cargo test --locked` passes (12 passed, 1 ignored). I haven't run the popup on a live panel.

---

From `2cb5cb58ca` to `7bac6b5767`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
