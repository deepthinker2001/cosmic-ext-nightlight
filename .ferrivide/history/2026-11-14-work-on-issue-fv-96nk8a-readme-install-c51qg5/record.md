# Work on issue FV-96NK8A: README: install from release, verify signature, badges

**Merged** 2026-11-14 · Engineer 1 (project agent) · `claude-sonnet-5-5` · branch `fv-96nk8a-readme-install-from-release-verify-signa` · FV-96NK8A: README: install from release, verify signature, badges

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-96NK8A: README: install from release, verify signature, badges
>
> Update README with the CI badge, installing the signed .deb, verifying the signature, the public key location, and how to cut a release.
>
> Labels: Release
>
> Much of the install/verify/release docs now exist in README from the .deb and release work. Review and tidy: add CI badge, a clear 'install from release' section, signature verification, how to cut a release, and how to create the signing key and fill in the fingerprint. Don't invent a fingerprint; keep the placeholder.

Then:

> [Event] Cyber reviewer (Opus 5.5) asks for changes: This change touches only the docs, and the instructions are mostly sound. The README's claim that the release workflow refuses to run while the placeholder is there is true (release.yml:40-45). Verification still goes through gpgv with only the downloaded key, and the fingerprint placeholder is kept. One problem: the new key-creation steps tell maintainers to print the armoured private key to the terminal. cargo test: 9 passed, 1 ignored.
> #1 [must fix] README.md:128: `gpg --armor --export-secret-keys KEYID` with nothing after it prints the whole private signing key to stdout. It then stays in terminal scrollback and in any tmux, IDE or session log, and the usual next step is copying it through the clipboard. The README also says to keep the revocation certificate offline but doesn't say where gpg put it. The same command is repeated in the secrets table at line 157. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Send the key straight into the environment secret so it never appears on screen: `gpg --armor --export-secret-keys KEYID | gh secret set GPG_PRIVATE_KEY --env release`. If `gh` isn't available, write it to a file with `(umask 077; gpg --armor --export-secret-keys KEYID > private.asc)`, paste it, then `shred -u private.asc`. Add that gpg saves the revocation certificate at `~/.gnupg/openpgp-revocs.d/<FINGERPRINT>.rev`, and tell maintainers to set a passphrase when `--quick-generate-key` asks. Change line 157 to match.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

> [Event] Your other reviewers' findings, to fix along with the changes asked for:
> #2 [suggestion] README.md:101: Line wrap in the Secrets section is broken: 'delete those tags). The build' ends a line early and the next line continues 'tools are installed...', so the paragraph reads raggedly. Reflow the paragraph to the same width as the rest of the file. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Rewrap the paragraph ending 'bump them by hand in a reviewed change.' so each line is filled like the surrounding text.
> #3 [suggestion] README.md:1: The CI badge still uses the OWNER/REPO placeholder, so it won't render until someone edits it. The repository has no git remote in this checkout, so the agent couldn't fill it in. Note this in the report, or fill in the GitHub path before merging. (Quick reviewer (Haiku 5.5))
> #4 [suggestion] README.md:3: The badge URL still contains the OWNER/REPO placeholder, so the badge and its link are broken until someone edits them. This is acceptable because the repository path isn't known here. Make sure the maintainer replaces it before publishing. (Sonnet 5.5)
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_unmerged_work); it's in files or an area another engineer is changing now (area_being_changed); it's too large for this change (too_large). A finding too large for this change needs an estimate. "Out of scope" isn't a reason. Your reviewers accept or reject each reason in the next round.

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `README.md` | modified | +68 −7 | Added CI badge, install-from-release section, signing key creation steps, release steps, and a note on where the public key is |

### Decisions

- **Badge uses OWNER/REPO placeholders**: The repository has no remote, so the GitHub path is unknown and I didn't want to invent one.
- **Kept the fingerprint placeholder and didn't repeat its token elsewhere**: The release workflow greps for that token and blocks the release while it's present.

## Tested

No test runs were reported.

Progress reports:

- README updated and committed: CI badge, an install-from-release section, key creation, release steps. The fingerprint placeholder is kept, and the badge still has OWNER/REPO to fill in. I didn't run anything.
- Fixed #1 and #2 in README.md and committed. #3 and #4 (the badge's OWNER/REPO placeholder) are declined, because the GitHub path isn't known. I didn't run anything.

## Reviewed

**Approved** by the cosmic-ext-nightlight reviewer (`claude-haiku-5-5`): Round 2 check. #1 (must_fix) is fixed: the signing-key section no longer prints the private key. It now pipes the export straight into `gh secret set ... --env release`, or writes it to a umask-077 file that is shredded afterwards, and it says where gpg saves the revocation certificate. The secrets table points to that section and says never to print the key. #2 is fixed: the Secrets paragraph is reflowed. #3 and #4 are accepted as needing a decision from the user: the GitHub owner/repo isn't known in this checkout, and only the maintainer can supply it. They go to a backlog issue. The rest of the change is unchanged since the last review, and I found nothing new on the changed lines.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-sonnet-5-5`): Finding #1 is fixed. The private key now goes straight into the secret through `gh`, or into a umask-077 file that is shredded afterwards. The docs also say where gpg saves the revocation certificate, tell maintainers to set a passphrase, and the secrets table was changed to match. The paragraph reflow (#2) is done. I accept the reasons for leaving #3 and #4: the badge's OWNER/REPO needs the maintainer's GitHub path, which only they can supply.

**Approved** by the cosmic-ext-nightlight reviewer (`claude-opus-5-5`): #1 is fixed. The private key now goes straight from gpg into `gh secret set`, or into a file only the maintainer can read that is then shredded. The revocation certificate's location and the passphrase are now covered, and the secrets table links to that section instead of repeating the command. The paragraph rewrap (#2) is done. I accept the deferrals of #3 and #4: only the user can give the GitHub path. cargo test: 9 passed, 1 ignored.

## Summary

I fixed findings #1 and #2 in `README.md` and committed the change. I didn't run anything, so the commands in the README are still untested.

- **#1, private key printing:** The README now pipes the key straight into the GitHub secret with `gpg --armor --export-secret-keys KEYID | gh secret set GPG_PRIVATE_KEY --env release`. Without `gh`, it writes the key to a file made under `umask 077` and then runs `shred -u` on it. It says never to print the private key. It also says to set a passphrase and gives the revocation certificate's location, `~/.gnupg/openpgp-revocs.d/<FINGERPRINT>.rev`. The secrets table now points to the key-creation section instead of repeating the print command.
- **#2, line wrap:** I rewrapped the paragraph at the end of the secrets section so the lines fill evenly.
- **#3 and #4, badge placeholder:** I'm declining both as `needs_user_decision`. The repository has no git remote, so I don't know the GitHub path for the CI badge. The maintainer has to replace `OWNER/REPO` in `README.md` before publishing, and a comment beside the badge says so.

---

From `9f5fb1a4a5` to `4d1566d716`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
