# Work on issue FV-J959H4: README: document settings

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-j959h4-readme-document-settings` · FV-J959H4: README: document settings

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-J959H4: README: document settings
>
> Update README.md: how to open the settings (right click), what the temperature and schedule settings do, and anything not verified on a live COSMIC session. Use the wording 'panel applet for COSMIC', not 'a COSMIC panel applet'.
>
> Labels: Docs
>
> The auto sunrise/sunset mode already added a README section. Check the README covers everything accurately now (right-click popup, temperature slider, custom schedule, auto mode and location) and fill only the gaps. Don't rewrite what's right.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `README.md` | modified | +33 −3 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): The change is README-only (README.md, plus the session record under .ferrivide/history). It adds a Settings section covering right-click open and close, the colour temperature slider, the custom schedule, the Auto pointer, and a "Not verified on a live COSMIC session" list, and it keeps the existing Auto section as it was. I checked its concrete claims against the code: the 1900–6500 K range and 100 K step (gamma.rs, main.rs slider .step(100)), the 3500 K default, the four hint bands (Very warm / Warm / Neutral warm / Cool, temperature_hint), the 21:00 and 07:00 defaults, the 20-second schedule tick, the midnight-crossing window and equal-times behaviour, and the right-click toggle. All match. The trademark wording uses "A panel applet for COSMIC", and no "COSMIC panel applet" remains. No code changed, so no build or tests were needed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): README-only change. The new Settings section and the "Not verified on a live COSMIC session" note match the code: right-click toggles the popup, the 1900–6500 K range and hint labels, the 20 s tick, and the schedule behaviour. The existing auto section is untouched, and the wording rule is kept. I didn't run the build or tests because no code changed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change only touches README.md (plus its session record). It adds no code, commands, links, dependencies or secrets, and it leaves the existing install and apt-key instructions as they were. The session record holds no credentials. cargo test --locked passes: 46 passed, 1 ignored.

## Summary

I added the missing settings documentation to `README.md` and committed it. I didn't build or run anything, since it's documentation only. I checked the new text against `src/main.rs` and `src/gamma.rs`.

The existing "Auto (sunrise/sunset)" section was accurate, so I left it as it was. These are the changes:

- **Stale line removed:** the README said the colour temperature "is not adjustable yet". It is adjustable now, so that line is replaced.
- **New "Settings" section:**
  - Right-clicking the button opens the popup, and right-clicking again closes it.
  - The temperature slider runs from 1900 K to 6500 K in 100 K steps, with a warm/cool hint. It defaults to 3500 K, and a change applies at once when night light is on.
  - The custom schedule has a start and end time, defaulting to 21:00 and 07:00 local time. The window can cross midnight, and equal times make an empty window.
  - The schedule is checked every 20 seconds and acts only when the window changes, so a manual click holds until the next start or end.
- **New "Not verified on a live COSMIC session" subsection:**
  - It lists the right-click popup opening in the panel and the popup layout.
  - It lists the real gamma output of the slider and the auto ramp, including on several monitors.
  - It lists behaviour after suspend and resume.

The wording "panel applet for COSMIC" is already used in the README, so I didn't need to change it.

---

From `b04ef61614` to `bd50bd26f2`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
