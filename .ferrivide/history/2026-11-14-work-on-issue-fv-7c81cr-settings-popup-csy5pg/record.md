# Work on issue FV-7C81CR: Settings popup on right click

**In progress** 2026-11-14 · Engineer 2 (project agent) · `claude-sonnet-5-5` · branch `fv-7c81cr-settings-popup-on-right-click` · FV-7C81CR: Settings popup on right click

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

Progress reports:

- Right-click settings popup with on/off switch added; builds and unit tests pass, not run on a live panel.

## Summary

Right-clicking the panel icon now opens a settings popup with a "Night light" on/off switch, and left click still toggles. I haven't run it on a live COSMIC panel. It builds with `--locked` and the 9 unit tests pass, but none of them covers the popup.

- **How right click works:** libcosmic's applet button only has left-press hooks, so I wrapped it in `mouse_area` with `on_right_press`. The popup itself is libcosmic's normal `app_popup`. Right-clicking again, or clicking away, closes it. The popup toggler sends the same toggle message as a left click.
- **Settings struct:** `Settings { enabled }` in `src/main.rs` loads and saves through cosmic_config, with the `enabled` key and config version 1 unchanged. I didn't bump the version, because a field added later reads as its default when the key is missing. I didn't use the `CosmicConfigEntry` derive, since it needs serde and macro paths this crate doesn't have. A comment on the struct says when a bump is needed.
- **Gamma:** unchanged at the fixed ~3500 K.
- **Not checked:** the popup anchors to the default rectangle from `get_popup_settings`, not the button's measured bounds. libcosmic's own example overrides that rectangle, but only with the bounds its left-press hook supplies, which the right-click path doesn't give. I'd check the popup's position on a real panel.

The change is committed with "Fixes FV-7C81CR".

---

From `2cb5cb58ca` to `36a06a9e74`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
