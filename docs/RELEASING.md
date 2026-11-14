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

## How a release is built

Publishing a GitHub release runs `.github/workflows/release.yml`. It has two
jobs, so that code from the build never shares a runner with the signing key:

- `build` checks out the release's tag, checks the tag equals `v` plus the
  `version` in `Cargo.toml`, builds the `.deb` with `just deb` (no cache; `just`
  and `cargo-deb` at fixed versions) and attaches it and its `.sha256` to the
  release. It has no secrets.
- `apt` holds the signing key. It downloads that `.deb`, checks its checksum,
  adds it to the apt repository on the `gh-pages` branch with
  `scripts/publish-apt.sh`, signs the repository and pushes the branch.
  GitHub Pages serves the branch at
  `https://deepthinker2001.github.io/cosmic-ext-nightlight`.

The workflow can also be run by hand from the Actions tab with a tag, to
redo the packaging of an existing release.

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

In the repository settings (or with `gh`):

```sh
gpg --armor --export-secret-keys FINGERPRINT | gh secret set APT_SIGNING_KEY
gh variable set APT_SIGNING_KEY_ID --body FINGERPRINT
```

- Secret `APT_SIGNING_KEY`: the ASCII-armoured private key. Never print it.
- Variable `APT_SIGNING_KEY_ID`: the key's fingerprint (not secret).

Also add a tag ruleset protecting `v*` (restrict who can create, update and
delete those tags). Only the `apt` job reads the secret.

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

## Cut a release

1. Make sure CI is green on `main`.
2. Set `version` in `Cargo.toml` (for example `0.2.0`) and commit the
   updated `Cargo.lock` (CI builds with `--locked`). A version is never
   reused: apt caches packages by version, so a fix is a new version.
3. Publish a GitHub release whose tag is `v` plus that version:
   `gh release create v0.2.0 --target main --title "0.2.0" --generate-notes`
   (or Releases → Draft a new release on GitHub). Don't attach a `.deb`
   yourself.
4. Watch Actions → Release: `build` and `apt` should both pass. Check the
   release lists the `.deb` and `.sha256`, and on a machine with the
   repository added, `sudo apt update && apt policy cosmic-ext-nightlight`
   shows the new version as the candidate.

If the workflow fails, fix the cause on `main` and rerun it for the tag:
`gh workflow run release.yml -f tag=v0.2.0`. The build uses the source at the
tag; the publishing script comes from `main`. If the fix has to be in the
source, make a new version instead.

## Not tested

None of this has run on GitHub: the workflow, `scripts/publish-apt.sh` with
real GitHub Pages, and the signed repository with a real apt client. The first
release is the first test, so after it, install from the repository on a clean
machine. `build` and `apt` use `ubuntu-latest`; the `.deb`'s libc6 minimum
comes from that runner.
