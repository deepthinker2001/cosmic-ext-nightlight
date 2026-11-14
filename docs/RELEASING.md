# Releasing (maintainers)

This page is for maintainers. Users: see the [README](../README.md).

## Build a .deb package locally

Install `cargo-deb` once (`cargo install --locked cargo-deb`; checked with
version 3.8.0), then:

```sh
just deb                # cargo deb --locked; builds the release binary too
sudo apt install ./target/debian/cosmic-ext-nightlight_*.deb
```

The package installs the same files as `just install` and depends on the
shared libraries the binary links (detected automatically), plus `cosmic-panel`.
It recommends `cosmic-comp`, which provides the gamma protocol the applet needs;
without it the toggle will not work. The libc6 minimum is taken from the
machine that builds the package, so build on the oldest release you want it to
install on.

### Built on the oldest supported distro: Ubuntu 24.04

The package must install on Ubuntu 24.04 (glibc 2.39) and newer. `cargo-deb`
sets the `libc6 (>= ...)` dependency from the glibc of the machine that builds
it, so a package built on a newer distro asks for a newer libc6 and apt refuses
it on 24.04. Version 0.2.0 shipped this way, built on `ubuntu-latest`, and needed
`libc6 (>= 2.43)`. So both the release `build` job and CI run on `ubuntu-24.04`,
never `ubuntu-latest`, and after `cargo deb` the release checks the package's
libc6 floor with `scripts/check-deb-libc.py`. The check fails the job if the
floor is above 2.39, or if the libc6 dependency is missing or in an unexpected
form. CI tests that check against sample packages
(`scripts/test-check-deb-libc.py`). When 24.04 stops being the oldest supported
release, change the runners and the 2.39 limit in the script together.

## How a release is built

A push to `main` runs `.github/workflows/release.yml`. It has three
jobs, so that code from the build never shares a runner with the signing key
or a write token:

- `build` has no secrets and a read-only token. It first reads the `version`
  in `Cargo.toml` (`cargo metadata --locked`). If the tag `v<version>` already
  exists, it prints a notice and the rest of the run is skipped: no release,
  no failure, so an ordinary push releases nothing. If the tag doesn't exist,
  this push is a release of `v<version>` at that commit: it builds the `.deb`
  with `just deb` (no cache; `just` and `cargo-deb` at fixed versions) and
  hands the `.deb` and its hash on. `attach` and `apt` run only for a release.
- `attach` holds the write token for the release. It runs no checkout and no
  cargo: it checks the artifact against the build's hash, creates the tag at
  the built commit's SHA and the GitHub release (generated notes, title =
  tag) with `gh release create --target <sha>`, and attaches the `.deb` and
  its `.sha256`. A release that already exists is left as it is, but the job
  first checks that the tag points at the built commit and stops if not. The tag is
  made with the workflow token, which does not start another workflow run.
- `apt` holds the signing key. It takes the `.deb` from the build job's
  artifact (not from the release, which could have been changed), checks it
  against the build's hash, adds it to the apt repository on the `gh-pages` branch with
  `scripts/publish-apt.sh`, signs the repository, checks that the exported
  `key.gpg` has the fingerprint in `APT_SIGNING_KEY_ID` and verifies both
  signatures with it (`gpgv`), and pushes the branch. A failed check stops
  the job before anything is pushed.
  GitHub Pages serves the branch at
  `https://deepthinker2001.github.io/cosmic-ext-nightlight`.

The workflow can also be run by hand from the Actions tab (from `main`) with
an existing tag, to redo the packaging of that release: it builds the tag's
source (the tag must be at a commit on `main`) and skips creating the release
if it exists.

The `gh-pages` branch holds only generated files: `pool/` (every published
`.deb`), `dists/stable/` (`Packages`, `Release`, `InRelease`, `Release.gpg`),
`key.gpg` (the public key) and `.nojekyll`. Never edit it by hand.

## One-time setup

### The signing key

On a trusted machine:

```sh
gpg --quick-generate-key "cosmic-ext-nightlight apt repository <you@example.com>" ed25519 sign 5y
gpg --list-keys --with-colons you@example.com   # the "fpr" line is the full fingerprint
```

The key must have no passphrase, because the workflow signs unattended (use
a key made only for this repository). gpg saves a revocation certificate at
`~/.gnupg/openpgp-revocs.d/<FINGERPRINT>.rev`; copy it and a backup of the
private key somewhere offline. Note the expiry date: extend the key before
then (`gpg --quick-set-expire FINGERPRINT 5y`, then update the secret and
rerun the workflow for the latest tag), or apt will refuse the repository.

### Secret and variable

Create a GitHub environment named `release` (Settings → Environments), add a
required reviewer, and limit its deployment branches and tags to the
branch `main` only (a rule of type **Branch**; no **Tag** rule). Releases and
manual reruns both run from `main`, so the `apt` job's ref is `main`: without
that rule GitHub rejects the job ("Branch "main" is not allowed to deploy to
release due to environment protection rules") and nothing is signed. Check
this under Settings → Environments → release → Deployment branches and
tags. Leaving out a `v*` Tag rule means a run started from a tag (a forged
one, say) can't reach the signing key at all. Put the
key in that environment, not in repository secrets, which every workflow run
can read:

```sh
gpg --armor --export-secret-keys FINGERPRINT | gh secret set APT_SIGNING_KEY --env release
gh variable set APT_SIGNING_KEY_ID --env release --body FINGERPRINT
```

- Environment secret `APT_SIGNING_KEY`: the ASCII-armoured private key.
  Never print it.
- Environment variable `APT_SIGNING_KEY_ID`: the key's fingerprint (not
  secret).

Also add a tag ruleset protecting `v*`: restrict who can create, update and
delete those tags to admins, with the GitHub Actions app as a bypass actor
(the workflow's `attach` job creates the tag, with the workflow token). If
the Actions app can't be given that bypass, create the tag some other way
that is not open to every collaborator (a deploy key or a GitHub App token
held by the `attach` job); don't open tag creation to everyone. This is the
control that makes every `v*` tag a commit that was on `main` when it was
released. The workflow also checks it: a manual rerun fails unless the tag's
commit is on `main`, and `attach` fails unless the tag points at the commit
that was built. Only the `apt` job uses the environment, so every release
(and every manual rerun) waits for the reviewer's approval before anything is
signed.

### The gh-pages branch and GitHub Pages

Create an empty branch with a first commit, once:

```sh
git switch --orphan gh-pages
git commit --allow-empty -m "Start the apt repository"
git push origin gh-pages
git switch main
```

Then in Settings → Pages, set the source to **Deploy from a branch**, branch
`gh-pages`, folder `/ (root)`. The first release fills the branch in.

### Protect `main`

The `apt` job runs `scripts/publish-apt.sh` from `main` with the signing key
loaded, and the reviewer approves a run, not the script's contents. So `main`
must have branch protection (a ruleset: no direct pushes, pull request reviews
required), and so should `gh-pages` against pushes by anyone but the workflow.

## Cut a release

A release is a commit on `main` that raises the version. There is no tag to
push and no release to create by hand. This intentionally replaces the earlier
tag-push release procedure: pushing a `v*` tag does not start a release. The
workflow creates the tag itself after building the pushed `main` commit; keep
the `release` environment limited to branch `main`, as described above.

1. Make sure CI is green on `main`. Before merging the version change, check
   that the signing key's full primary fingerprint (`gpg --list-keys
   --with-colons --fingerprint` on the trusted signing machine) and the
   `release` environment's `APT_SIGNING_KEY_ID` both match the fingerprint
   published in the [README verification steps](../README.md#with-apt-recommended):
   `0472CEF01CE0E79EC118662D34A06EB979E58584`. Ignore spaces when comparing
   the README's grouped form. If they differ, stop and resolve the mismatch
   before releasing; do not replace the documented fingerprint just to match
   a downloaded key.
2. In a change to `main`, set `version` in `Cargo.toml` (for example `0.2.0`)
   and commit the updated `Cargo.lock` (CI builds with `--locked`). A version
   is never reused: apt caches packages by version, so a fix is a new
   version. Tags that already exist are spent too: a push whose version's tag
   exists releases nothing.
   In the same commit, rename the `Unreleased` heading in `CHANGELOG.md` to
   the version and date, and add an empty `Unreleased` above it.
3. Merge it and `git push` `main`. The workflow sees a version without a tag
   and makes the release.
4. Approve the `release` environment when GitHub asks (the `apt` job waits
   for it). Watch Actions → Release: `build`, `attach` and `apt`
   should all pass. Check the release (tag `v0.2.0`, at the pushed commit)
   lists the `.deb` and `.sha256`, and on a machine with the
   repository added, `sudo apt update && apt policy cosmic-ext-nightlight`
   shows the new version as the candidate.

If the workflow fails after `attach` made the tag, the tag is spent and a
new push won't release again. Fix the cause on `main` and rerun the packaging
for that tag: `gh workflow run release.yml --ref main -f tag=v0.2.0`. The
build uses the source at the tag; the publishing script comes from `main`. If
the fix has to be in the source, make a new version instead. If `build` fails
before `attach`, no tag exists: fix `main` and push again.

## Not tested

None of this has run on GitHub: the workflow, `scripts/publish-apt.sh` with
real GitHub Pages, and the signed repository with a real apt client. The first
release is the first test, so after it, install from the repository on a clean
machine. `build` and `apt` use `ubuntu-latest`; the `.deb`'s libc6 minimum
comes from that runner.
