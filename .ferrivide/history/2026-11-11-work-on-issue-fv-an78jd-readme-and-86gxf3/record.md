# Work on issue FV-AN78JD: README and install instructions

**In progress** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-an78jd-readme-and-install-instructions` · FV-AN78JD: README and install instructions

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-AN78JD: README and install instructions
>
> Document what the applet does, what it needs (a compositor with wlr-gamma-control, which cosmic-comp provides), how to build and install it with just, and how to add it to the panel.
>
> Labels: docs

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [suggestion] README.md:17: The list of development libraries (libxkbcommon, libwayland, libinput, libudev, libssl) is not checked against libcosmic's actual requirements. A reader may install the wrong packages or miss some. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Check the list against libcosmic's documented build dependencies, or say it is an example list and link to libcosmic's requirements.
> #2 [suggestion] README.md:42: The panel menu labels (Settings → Desktop → Panel, Configure panel applets, Configure dock applets) are written from memory and were not confirmed on a running COSMIC session. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Confirm the menu wording on a running COSMIC session, or soften it to describe the panel settings generally.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `README.md` | modified | +55 −0 |  |

## Tested

No test runs were reported.

## Summary

I replaced the one-line `README.md` with a full one and committed it. I didn't build or run anything, so the commands and panel steps are untested.

The README covers:
- **What it does:** a panel toggle that sets a fixed ~3500 K tint through the Wayland protocol and remembers the on/off state across restarts.
- **What it needs:** a compositor with `wlr-gamma-control`, which `cosmic-comp` provides. Without it the icon stays off and the error goes to stderr.
- **Build and install:** `just build`, `sudo just install` and `sudo just uninstall`, plus where the files land under `/usr`.
- **Adding it to the panel:** open the panel or dock applet settings and add Night Light.

Things to check:
- **Build libraries:** the list of development libraries needed to build is my guess from libcosmic's usual dependencies, not checked against a build.
- **Panel steps:** the Settings menu path for adding the applet is written from memory of COSMIC's UI, and so is the `pkill cosmic-panel` restart hint.
- **`just prefix=$HOME/.local install`:** I wrote it as an alternative, but I haven't confirmed it works with the `justfile`. I also said the panel may not find applets outside `/usr`, which I didn't verify either.

---

From `55627f842e` to `4139b8ee7b`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
