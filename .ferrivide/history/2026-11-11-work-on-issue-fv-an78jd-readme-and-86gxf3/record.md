# Work on issue FV-AN78JD: README and install instructions

**Merged** 2026-11-11 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-an78jd-readme-and-install-instructions` · FV-AN78JD: README and install instructions

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

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Both round-one findings are fixed in 67db758. The library list now points to libcosmic's build dependencies instead of an unverified list, and the panel menu steps are softened with a note that names may differ between COSMIC versions. The changed lines are documentation only and contain no defects, so nothing blocks the merge.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Both earlier suggestions are fixed: the library list now links to libcosmic, and the panel steps are generic.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The only change since round 1 is wording in README.md, and both earlier suggestions are handled. The library list now points to libcosmic's official repository over https. The panel steps are described generally and say the menu names may differ between COSMIC versions. Nothing new has a security bearing: no commands to paste, and the `pkill` hint was replaced by logging out and back in. `cargo test` passes: 6 passed, 1 ignored.

## Summary

I fixed both review findings in `README.md` and committed. I didn't build or run anything, so the commands and panel steps in the README are still untested.

- **#1, library list:** I removed my guessed list of development libraries. The README now says the list depends on your distribution and links to the libcosmic repository for its current build dependencies. It also tells the reader to install whatever the build reports as missing.
- **#2, panel menu wording:** I replaced the specific menu path with a general instruction. It now says to open COSMIC Settings, go to the panel or dock settings, and use the applet configuration to add Night Light. It notes that the menu names may differ between COSMIC versions. I also dropped the `pkill cosmic-panel` hint and now only suggest logging out and back in.

I also removed the `just prefix=$HOME/.local install` example, which I hadn't verified. The README now says the default `/usr` is sure to be searched. It adds that the `justfile` has a `prefix` variable and that the panel only finds applets on the XDG data path.

---

From `55627f842e` to `67db758aef`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
