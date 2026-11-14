# Work on issue FV-SKA1WX: Audit: what the reviews of Engineer 2's work on…

**In progress** 2026-11-14 · Engineer 2 (project agent) · `claude-sonnet-5-5` · branch `fv-ska1wx-audit-what-the-reviews-of-engineer-2-s-w` · FV-SKA1WX: Audit: what the reviews of Engineer 2's work on cosmic-ext-nightlight left for later (Applet)

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-SKA1WX: Audit: what the reviews of Engineer 2's work on cosmic-ext-nightlight left for later (Applet)
>
> Findings that couldn't be resolved in that change (R24), each with why:
>
> - [suggestion] src/main.rs:160: `Settings::load` and `save_enabled` have no test. A small test of the default value would be worthwhile.
>   - Why it's left for later: a suggestion raised in a re-review round, which can't hold the merge
>   - Raised by Sonnet 5.5 on Engineer 2's work on cosmic-ext-nightlight
>
> Labels: audit, Applet

Then:

> [Event] Your reviewers approve, with findings to fix in this change before it merges:
> #1 [suggestion] src/main.rs:475: The round-trip assertion sits behind `if Settings::save_enabled(...).is_ok()`, so if saving fails the test still passes after checking only the defaults. The cleanup also runs only if the test gets that far, and it copies cosmic_config's config-path logic by hand (dirs_config_dir), which could drift from the real path. (Quick reviewer (Haiku 5.5))
>    Suggested fix: Either assert that save_enabled succeeds when Config::new succeeds, or accept the skip explicitly and say so in the test's comment. Consider a drop guard for the cleanup, and reuse the library's path function if cosmic_config exposes one.
> #2 [suggestion] src/main.rs:472: The test returns early when Config::new fails and skips the round trip when save fails, so it can pass without checking anything. It also leaves a stray config dir if the first assert fails, because cleanup comes after it. Both are minor. (Sonnet 5.5)
> #3 [should fix] src/main.rs:475: The test calls `Config::new`, which writes into the real user config directory (`~/.config/cosmic/<APP_ID>.test<pid>/v1`). It then deletes that folder with `remove_dir_all`, using a path it rebuilds itself in `dirs_config_dir` (line 486). That rebuilt path doesn't match how cosmic-config resolves the directory. cosmic-config honours `FLATPAK_ID`/`HOST_XDG_CONFIG_HOME`, and its `dirs::config_dir` ignores an empty or relative `XDG_CONFIG_HOME`, while the helper accepts any value. When the two differ, the test leaves a folder in the user's real config, and the delete aims at a path relative to the working directory. If an assertion fails, the cleanup never runs. A later run that gets the same pid then loads the leftover file and fails `load == default` for no real reason. Also, when `Config::new` or `save_enabled` fails, the test returns early or skips its assertion, so it can pass without checking anything. (Cyber reviewer (Opus 5.5))
>    Suggested fix: Use cosmic-config's own isolation instead of the real home. Use `let dir = tempfile::tempdir()?;` (add `tempfile` as a dev-dependency; it's already in Cargo.lock), then `let config = Config::with_custom_path(APP_ID, CONFIG_VERSION, dir.path().to_path_buf()).expect("config");`. The temp folder gets a random name and 0700 permissions and is removed on drop, even after a panic. Delete `dirs_config_dir` and the `remove_dir_all` call. Use `expect` instead of the early `return` and the `if ... is_ok()`, so a failed save makes the test fail.
> Fix each of these in this branch, whatever its severity (findings are fixed in the change that raised them, R24), and commit; your reviewers then check the fixes. Leave one for later only if it can't be resolved in this change: name it in report_status's `declined` (its number, the reason, and the detail), with one of these reasons: it needs a decision from the user (needs_user_decision); it depends on other work that isn't merged yet (waits_on_u…

(Shortened here; record.json has all of it.)

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `Cargo.lock` | modified | +1 −0 |  |
| `Cargo.toml` | modified | +3 −0 |  |
| `src/main.rs` | modified | +11 −0 |  |

## Tested

No test runs were reported.

## Summary

Replace lines 472–494.

## Contract

The clauses this work was bound by:

1. C2: Before finishing, every engineer runs `cargo fmt --all -- --check`, and `cargo clippy --all-targets` with no new warnings, besides the tests.
2. C4: Commit messages describe the change itself (what and why) and never name Ferrivide agents such as "Engineer 1", "Merge engineer 1", or agent branches.

---

From `626aa5bb40` to `a242028de6`. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
