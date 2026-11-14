# Work on issue FV-A70TWP: GitHub Actions CI

**In progress** 2026-11-14 · Engineer 2 (project agent) · `claude-sonnet-5-5` · branch `fv-a70twp-github-actions-ci` · FV-A70TWP: GitHub Actions CI

About $0.00, 0 tokens in, 0 out

## Asked

> Work on issue FV-A70TWP: GitHub Actions CI
>
> Add a workflow run on push and pull request: cargo fmt --check, clippy, tests and a locked build, using the libcosmic system dependencies. Match the user's other projects' workflows where they are available.
>
> Labels: Release

## Implemented

| File | Change | Lines | Why |
|---|---|---|---|
| `.github/workflows/ci.yml` | added | +41 −0 |  |
| `src/gamma.rs` | modified | +34 −8 |  |
| `src/main.rs` | modified | +16 −3 |  |

## Tested

No test runs were reported.

## Summary

Clippy with `-D warnings` is clean.

---

From `806260f1f1` to uncommitted work. The whole diff is in [changes.diff](changes.diff). The conversation, shortened, is in [transcript.jsonl](transcript.jsonl); [the README](../README.md) explains the files.
