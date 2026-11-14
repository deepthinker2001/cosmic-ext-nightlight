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

Pushing a tag like `v0.1.0` (it must equal the `version` in `Cargo.toml`) runs
`.github/workflows/release.yml`. It builds the `.deb` with `just deb` in a job
that has no secrets, signs it in a separate job, and publishes a GitHub Release
in a third. The release contains:

- `cosmic-ext-nightlight_*.deb`
- `cosmic-ext-nightlight_*.deb.asc` (detached GPG signature)
- `SHA256SUMS` and `SHA256SUMS.asc` (the checksums, clear-signed)
- `signing-key.asc` (the public key, for convenience only)

## Create the signing key (once)

On a trusted machine:

```sh
gpg --quick-generate-key "cosmic-ext-nightlight releases <you@example.com>" ed25519 sign 2y
gpg --list-keys --with-colons --fingerprint you@example.com   # the "fpr" line is the full fingerprint
```

Set a passphrase when gpg asks for one. gpg also saves a revocation certificate
at `~/.gnupg/openpgp-revocs.d/<FINGERPRINT>.rev`; copy it somewhere offline.

Send the private key straight into the environment secret (after creating the
`release` environment, see [Secrets](#secrets)), so it never appears on screen
or in scrollback:

```sh
gpg --armor --export-secret-keys KEYID | gh secret set GPG_PRIVATE_KEY --env release
```

Without `gh`, write it to a protected file, paste it into GitHub's secret form,
and destroy the file:

```sh
(umask 077; gpg --armor --export-secret-keys KEYID > private.asc)
shred -u private.asc
```

Never print the private key to the terminal. Paste the full 40-character
fingerprint into the code block under
[Verify a download](../README.md#verify-a-download) in the README, replacing the
placeholder line. The release workflow refuses to run while that placeholder
(`FINGERPRINT-TO-BE-FILLED-IN`) is still in `README.md`, so keep it in the
README's fingerprint block until then. Keep a backup of the private key and its
revocation certificate offline. The key expires after two years: extend it or
make a new one before then, and update the fingerprint in the README if it
changes.

## Secrets

The signing key is never committed. Create a GitHub environment named
`release` (Settings → Environments), limit its deployment branches and tags to
`v*`, add a required reviewer, and add these as **environment secrets** of that
environment (not repository secrets, which every workflow run can read):

| Secret | Contents |
| --- | --- |
| `GPG_PRIVATE_KEY` | ASCII-armoured private signing key (export it as in [Create the signing key](#create-the-signing-key-once); never print it) |
| `GPG_KEY_ID` | Key ID or fingerprint to sign with |
| `GPG_PASSPHRASE` | The key's passphrase (leave empty if it has none) |

Also add a tag ruleset protecting `v*` (restrict who can create, update and
delete those tags). The build tools are installed at fixed versions (`just`
1.59.0, `cargo-deb` 3.8.0) in the workflow; bump them by hand in a reviewed
change.

## Cut a release

1. Make sure the fingerprint is filled in and CI is green on `main`.
2. Set `version` in `Cargo.toml` (and commit the updated `Cargo.lock`).
3. Tag and push: `git tag v0.1.0 && git push origin v0.1.0` (the tag must be
   `v` plus the `Cargo.toml` version).
4. Approve the run of the `release` environment when GitHub asks.
5. Check the new release has all five files and that you can verify a download
   from it, following [the README](../README.md#verify-a-download).
