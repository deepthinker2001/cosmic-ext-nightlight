# You are cosmic-ext-nightlight's Release pass, for this one job: nobody else is…

**In progress** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `ferrivide/release-pass-fv-6f5h0e`

About $0.00, 0 tokens in, 0 out

## Asked

> You are cosmic-ext-nightlight's Release pass, for this one job: nobody else is working here now, so what you change can't collide with anyone. The user has asked for a release. You work like an engineer, in your own worktree and branch; what you change is reviewed and merged like any other work. Check four things, fix what you can in your branch, and file an issue (create_issue) for each thing you can't fix in this one pass. You don't build, sign or publish the release: that is the user's, and you never read, copy or ask for a private signing key.
>
> 1. Signing. Check against the project's requirements (the ide tool's requirements action; R14 asks that release builds be signed and that users can verify the signatures) that the release scripts sign every build they produce, that the public keys the project ships verify what they sign, and that the project's own verification (its release tool's verify command, where it has one) passes on the latest signed output.
> 2. Licenses. Check against R33 that no build contains GPL, LGPL or AGPL code, including code generated from GPL-family sources (read the dependency licenses, the vendored code and what is linked in), and that every bundled font, model and asset has a clear license listed with its owner and notice, as the project's list of bundled assets and its third-party notices require.
> 3. Vulnerabilities. Check against R34 that no critical or actively exploited finding is left unaddressed: read the CVE tab's findings (the ide tool's vulnerabilities action); Compliance runs the scanners, so you don't. A finding is addressed when it is fixed, or when a person has accepted it as an exception; you never accept one yourself. Name each open critical or actively exploited finding in your report as a blocker.
> 4. Release notes. Check that the release notes (the change log and the notes in the docs) match what merged since the last release: read the git log from the last release's tag or commit, and correct a note that is missing, wrong or about work that isn't in.
>
> Leave alone what's another job: no new features, no speed work, no redesign. Run a command heavier than the default resource limits only after asking through C13.
>
> Your closing report_status is published as your report, in the project's Reports tab (Release Pass). Put in its summary a section for each: Signing, Licenses, Vulnerabilities and Release notes (what you checked, what you fixed, what's filed), and begin it with one line: ready to release, or the blockers that remain. List your changes in its decisions and files. If nothing needs changing, commit nothing and say what you checked. Once your work is merged (or you finish with nothing to merge), you're discarded.
>
> The user's direction for this start, as the project manager carries it (it comes first where it narrows the work):
> Release

Then:

> [Ferrivide] Your helper's answer to "In this repo (cosmic-ext-nightlight), where did data/icons/io.github.cosmic_ext.Nightlight-on-symbolic.svg and -off-symbolic.svg come from: hand-drawn by an agent, or copied/adapted from a third-party icon set (Adwaita, Pop, cosmic-icons, Material, etc.)? Check the SVG contents (metadata, comments, ":
>
> **Short answer: the icons were hand-written by an agent, not copied from a third-party set.** I found no third-party markers in them, and no notices or attribution text in the repo.
>
> **SVG contents** (`data/icons/`)
> - `io.github.cosmic_ext.Nightlight-on-symbolic.svg`: one `<path>` with a hand-typed `d=` (a circle-with-notch crescent), `fill="#bebebe"`. No metadata, comments, `<title>`, Inkscape/Sodipodi tags, or `<defs>`. The file is a single line.
> - `io.github.cosmic_ext.Nightlight-off-symbolic.svg`: a crescent path at `opacity=".4"` plus a diagonal bar path (`M2 3l1-1 11 11-1 1z`). Same bare format, no metadata.
>
> **Git / history**
> - The reflog for `/home/mad/projects/cosmic-ext-nightlight/.git/logs/HEAD` contains no SVG-specific entries. Commit `5eff73e "Scaffold libcosmic panel applet"` is the commit that first contains these files (per the review-1 output in the scaffold session). I did not run git, so I haven't confirmed the blob directly.
> - Session `.ferrivide/history/2026-11-11-work-on-issue-fv-19xwhq-scaffold-the-b2xyj9/`:
>   - `transcript.jsonl` line 11 writes both SVGs with `cat > ... <<'E'` heredocs. The agent typed the markup itself.
>   - `transcript.jsonl` line 22 runs `sed -i 's/1\.400/1.4/g;...' data/icons/*.svg`, which only normalises decimal numbers in the path data.
>   - `changes.diff` lines 6885–6898 show both as "new file", one line each.
>   - `record.md` lines 22–23 list both as "added".
> - Nothing in any session mentions Adwaita, Pop, cosmic-icons, Material, or another icon set as a source. Searching `.ferrivide` for "icon" found no reference to a source or template.
> - Later sessions only reference the icon names (`src/main.rs:23`, `data/io.github.cosmic_ext.Nightlight.desktop:9`, `Cargo.toml:30`). None of them modify the SVGs.
>
> **Notices, asset lists, licence text**
> - Searched for "Adwaita", "THIRD", "NOTICE", "attribution", and "licen" across the repo, `docs/`, and `README.md`. The only hits are in `LICENSE` (the GPL-3.0 text, which is the project licence in `Cargo.toml:6`) and generic words in it.
> - There is no third-party notices file, asset list, or per-asset attribution anywhere. `docs/` contains only `RELEASING.md`, which has no licence mention.
> - `data/` contains only `icons/` and the `.desktop` file.
> - `src/` contains Rust code. The search found no copyright headers there. The copyright and SPDX headers in `.ferrivide/history/2026-11-14-work-on-issue-fv-fk5e4x-automatic-schedule-gxh4v6/transcript.jsonl` (lines 13, 19) belong to libcosmic/iced files that an agent read, not to this repo's own files.
>
> **Not verified / gaps**
> - I didn't check whether the two SVGs have the same blob hash in …

(Shortened here; record.json has all of it.)

## Understood

Release pass for cosmic-ext-nightlight: check signing (R14), licenses (R33), vulnerabilities (R34) and release notes against what merged since the last release; fix what I can in this branch, file an issue for each thing I can't, and report blockers. No build, signing or publishing.

Assumptions:

- R14, R33 and R34 aren't in the project's recorded requirements (only R0–R2 are); I check against them as the job describes them.
- The last release is v0.1.0: it's the only one that produced a .deb. v0.1.1 is tagged, but its build failed at the tag/version check, so nothing shipped from it.
- Dropping per-.deb signatures in favour of the signed apt repo was the user's own direction (FV-QVWSWT), so I file the unsigned release-page download rather than reversing that decision.
- Choosing the project's license, and whether R33 should bend for the GPL cosmic-panel crates libcosmic's applet feature needs, are the user's decisions.

Plan:

- scripts/publish-apt.sh: after signing, check that the exported key.gpg has the fingerprint given and that gpgv verifies InRelease and Release.gpg with it; test it with a throwaway key in scratch.
- .gitignore: ignore private-key files (one sits untracked in the main checkout).
- Add CHANGELOG.md with an Unreleased section for the work since v0.1.0, plus v0.1.0; add a CHANGELOG step to docs/RELEASING.md, and say that the release environment's v* rule must be a tag rule and that 0.1.1 is spent.
- File issues: license conflict with R33, missing third-party notices and asset list, unsigned release-page .deb, the release environment rejecting tags, and the CVE scan never having run.
- Report the blockers.

Out of scope:

- Changing the license or replacing libcosmic dependencies
- Setting the release version or tagging
- Running vulnerability scanners (Compliance's job)
- Changing GitHub settings
- Generating full dependency notices

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.gitignore` | modified | +5 −0 |  |
| `CHANGELOG.md` | added | +53 −0 |  |
| `README.md` | modified | +2 −0 |  |
| `docs/RELEASING.md` | modified | +13 −3 |  |
| `scripts/publish-apt.sh` | modified | +16 −0 |  |

## Tested

No test runs were reported.

## Summary

No tracked file falls under the new ignore rules. No Rust code changed, so the real check is the `publish-apt.sh` test above. Committing:

---

From `35b77c73ef` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
