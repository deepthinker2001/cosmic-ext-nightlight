# You are cosmic-ext-nightlight's Release pass, for this one job: nobody else is…

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-opus-5-5` · branch `ferrivide/release-pass-fv-6f5h0e`

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

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [suggestion] scripts/publish-apt.sh:49: With a wrong or short key id, the earlier `gpg --local-user` signing or `gpg --export` fails under `set -e` before the new fingerprint message can print, so the friendly message only appears when the id resolves to a key whose fingerprint differs from the one given. The check still fails closed, so this is cosmetic. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Optional: move the fingerprint comparison to right after the key is chosen, or note in the usage line that <key-id> must be a full fingerprint.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

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
| `.gitignore` | modified | +5 −0 | Never commit a private signing key (private-key*, *.key, *.asc) |
| `CHANGELOG.md` | added | +53 −0 | New change log: Unreleased (everything since v0.1.0) and 0.1.0 |
| `README.md` | modified | +2 −0 | Link to CHANGELOG.md |
| `docs/RELEASING.md` | modified | +13 −3 | The v* environment rule must be a Tag rule; spent tags; update CHANGELOG when setting the version; describe the signature check |
| `scripts/publish-apt.sh` | modified | +30 −3 | Check the full fingerprint against the keyring before signing; usage line says <key-id> must be the full fingerprint |

### Decisions

- **Added a key/signature self-check to publish-apt.sh, not a separate verify step in the workflow**: The script is what signs, so a check there runs in every path (CI and by hand) before anything is pushed, and it can be tested locally with a throwaway key. Turned down: A separate workflow step verifying gh-pages after push (too late: users would already see a broken repo).
- **Didn't add signatures for the release-page .deb; filed FV-KRNV6Q**: The user asked in FV-QVWSWT to drop the per-.deb gpgv flow, so changing that is their call. Turned down: Sign the .deb in the apt job and upload an .asc.
- **Didn't change LICENSE or Cargo.toml's license; filed FV-9CWESN**: Choosing the license, and whether R33 makes an exception for the GPL crates libcosmic's applet feature needs, are the user's decisions.
- **The change log treats everything since v0.1.0 as Unreleased**: v0.1.1 is tagged, but its build failed and shipped no package, so none of that work has reached users; the next version must be new (0.1.1 is spent). Turned down: A 0.1.1 entry (it would describe a release that doesn't exist).
- **Didn't bump the Cargo.toml version**: The version number is the user's choice when they cut the release; RELEASING.md and FV-V9V00C say it must be new.

## Tested

- `scripts/publish-apt.sh run against a throwaway key and a dummy .deb: matching fingerprint, short key id, signed with another key`: 3 passed, 0 failed
- `scripts/publish-apt.sh with a throwaway key: good fpr, short id, unknown fpr, other signer`: 4 passed, 0 failed

Progress reports:

- NOT ready to release. Blockers: (1) R33 — every build contains GPL code: libcosmic's applet feature links GPL-3.0-only cosmic-panel-config / xdg-shell-wrapper-config, and the project itself is GPL-family (Cargo.toml says GPL-3.0-only, LICENSE is AGPL-3.0) — FV-9CWESN, user decision; (2) R34 — the CVE tab has never been scanned, so nothing shows the absence of critical or exploited findings — FV-X6J1CF, Compliance; (3) R14 — no build has ever been signed: the GitHub `release` environment rejected v0.1.0's signing job, and v0.1.1's build failed its version check — FV-V9V00C, GitHub setting; (4) third-party notices for the embedded OFL fonts and dependencies are missing — FV-W86KWZ.

## Signing (R14)
Checked: release.yml and publish-apt.sh sign the apt repository (InRelease, Release.gpg) after checking the build's hash; the public Actions API for both releases; gh-pages; the README's fingerprint steps. Nothing signed exists to verify: gh-pages holds only its empty first commit. v0.1.0's `apt` job was refused ("Tag v0.1.0 is not allowed to deploy to release due to environment protection rules"). v0.1.1 failed at "Check the tag matches the version" (Cargo.toml still says 0.1.0). The project has no release tool with a verify command. Fixed: publish-apt.sh now checks that the shipped key.gpg has the fingerprint in APT_SIGNING_KEY_ID and verifies both signatures with gpgv before anything is pushed. I tested it with a throwaway key: correct key passes, short key id fails, signing with a different key fails. RELEASING.md now says the v* environment rule must be of type Tag. Filed: FV-V9V00C (environment setting, next version number), FV-KRNV6Q (the release-page .deb has only a .sha256 from the same release, a trade-off the user chose in FV-QVWSWT).

## Licenses (R33)
Checked all 527 packages in the Linux build (cargo tree, license fields, and LICENSE files in the git checkouts), and the fonts and icons compiled in. GPL found: the project's own license, and cosmic-panel-config / xdg-shell-wrapper-config (GPL-3.0-only, required for R1). configparser and self_cell are dual-licensed and fine under their MIT/Apache options; the MPL-2.0 crates are not GPL. Bundled fonts: Open Sans and Noto Sans Mono (OFL-1.1, from libcosmic) and Iced-Icons.ttf (iced, MIT, origin to confirm). The two SVG icons are the project's own, hand-written in the scaffold session. There is no notices file or asset list. Filed FV-9CWESN and FV-W86KWZ; changing the license is the user's choice.

## Vulnerabilities (R34)
The CVE tab: scanned_at null, no findings, no scanners, exploitation status unknown. No finding is open, but none has been looked for. Filed FV-X6J1CF for Compliance. I accepted nothing.

## Release notes
There was no change log. GitHub's v0.1.1 notes are only a compare link, for a build that shipped nothing. Added CHANGELOG.md: an Unreleased section with the settings popup, temperature slider, custom schedule, sunrise/sunset auto ramp, the trademark rewording and the apt signature check, plus 0.1.0. Also added a RELEASING.md step to roll the Unreleased section into each new version, and a README link.

Also: a `private-key.asc` sits untracked in the user's main checkout (never committed; I didn't open it). .gitignore now excludes key files. It would be safer moved out of the project folder. FV-CK2Y04 and FV-B4S568 (fingerprint and badge placeholders) are already resolved on main and can be closed. R14, R33 and R34 aren't in the project's recorded requirements (only R0–R2).
- Fixed review finding #1 (publish-apt.sh checks the key's full fingerprint before signing; usage says so) and committed. Tested with a throwaway key: correct key passes; short id and unknown fingerprint stop with the message before signing; signing with another key fails at gpgv. The release blockers from my report stand: FV-9CWESN (GPL / R33), FV-X6J1CF (no CVE scan / R34), FV-V9V00C (environment rejects tags, so nothing has been signed / R14), FV-W86KWZ (no third-party notices).

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Finding #1 is fixed. The fingerprint check now runs before anything is signed, against the secret keys in the keyring, so a wrong or short key id fails with the friendly message. I ran the lookup against the scratch test key: the full fingerprint in either letter case matches, while an email address and a made-up id are rejected. The later check on key.gpg still runs. The only change since the last review is in scripts/publish-apt.sh.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): The finding is fixed. The fingerprint check now runs before any signing, and the usage line and header say the key id must be a full fingerprint. The script passes a `sh -n` syntax check. I didn't run it.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): The quick reviewer's suggestion #1 is fixed. The key's fingerprint is now checked against the secret keyring before anything is written or signed, and the usage line says a full fingerprint is needed. The check fails closed: `--` comes before the key id, and a subkey or partial match gives a mismatch. I ran it against an empty keyring: it exited 1 with the message and created nothing. The check after signing, on the exported key.gpg and with gpgv, is unchanged.

## Summary

I fixed the reviewer's finding and committed it. The project is still **not ready to release**: the four blockers from my first report haven't changed.

**The fix:** `scripts/publish-apt.sh` now checks the key's full fingerprint at the very start, before anything is signed. Before, a short or unknown key ID could stop the script inside a gpg command without the clear message. The usage line now says the key ID must be the full fingerprint. The checks after signing are unchanged: the published `key.gpg` must have that fingerprint and verify both signatures with gpgv.

**Tests**, run with a throwaway key:
- The correct fingerprint passes, with spaces and lower case accepted.
- A short key ID and an unknown fingerprint both stop with the clear message before anything is signed.
- Signing with a different key still fails when the signatures are verified.

**Blockers still open:**
- **FV-9CWESN (licenses, R33):** every build contains GPL code, and the LICENSE file and Cargo.toml disagree on the project's own license. Both need your decision.
- **FV-X6J1CF (vulnerabilities, R34):** the CVE tab has never been scanned; Compliance needs to run the scanners.
- **FV-V9V00C (signing, R14):** your GitHub `release` environment rejects `v*` tags, so no release has ever been signed.
- **FV-W86KWZ:** the package has no third-party notices for the bundled fonts and dependencies.

---

From `35b77c73ef` to `b6975d2dfb`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
