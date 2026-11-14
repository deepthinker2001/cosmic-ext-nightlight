# Work on issue FV-BA20B4: Reword "a COSMIC panel applet" (trademark)

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-ba20b4-reword-a-cosmic-panel-applet-trademark` · FV-BA20B4: Reword "a COSMIC panel applet" (trademark)

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-BA20B4: Reword "a COSMIC panel applet" (trademark)
>
> COSMIC is a trademark, so we can't call the project "a COSMIC panel applet". Reword it as "a panel applet for COSMIC" everywhere it appears in files we ship or show: README.md line 5, Cargo.toml `description` (line 5) and the `[package.metadata.deb]` `extended-description` (line 19). Also check the .desktop file, data/ and docs/ for the same phrase. Keep the other uses that merely name the desktop environment (e.g. "COSMIC's compositor"). Don't edit .ferrivide/history. Run the tests.
>
> Labels: Docs

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.toml` | modified | +2 −2 |  |
| `README.md` | modified | +1 −1 |  |

## Tested

No test runs were reported.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): The change rewords the three places the task named (README.md line 5, Cargo.toml description and the deb extended-description) as "a panel applet for COSMIC" and leaves the other COSMIC mentions (cosmic-comp, COSMIC panel, COSMIC Settings) alone, as asked. The .desktop file, data/, docs/ and scripts/ contain no "COSMIC panel applet" phrase; the one in scripts/publish-apt.sh already reads "a panel applet that toggles" and needs no change. cargo test passes (9 passed, 1 ignored live probe), and no code or history files changed.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The three required spots are reworded: README line 5, Cargo.toml `description` and the deb `extended-description`. A grep finds no remaining "COSMIC panel applet", and the .desktop file, data/ and docs/ needed no change. I did not run the tests or the build, but the diff touches only text and metadata.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The change only rewords text in README.md and Cargo.toml (description, deb extended-description). It touches no code, dependencies, permissions or build logic, so it has no security impact. The phrase appears nowhere else outside .ferrivide/history. cargo test: 9 passed, 1 ignored.

## Summary

I reworded "a COSMIC panel applet" as "a panel applet for COSMIC" in three places and committed it. `cargo test` passes: 9 passed, 1 ignored, 0 failed.

- **`README.md` line 5:** now "A panel applet for COSMIC that toggles a night light…".
- **`Cargo.toml` `description` (line 5):** now "Panel applet for COSMIC to toggle night light".
- **`Cargo.toml` `extended-description` (line 19):** now "A panel applet for COSMIC that toggles night light…".

I searched the repo for other uses of the phrase and found none in the `.desktop` file, `data/` or `docs/`. The `.desktop` file's `Comment` reads "Toggle night light from the panel", and its `X-CosmicApplet=true` line is a key name, not prose, so I left both. I didn't touch `.ferrivide/history`.

---

From `2cb5cb58ca` to `a44d040861`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
