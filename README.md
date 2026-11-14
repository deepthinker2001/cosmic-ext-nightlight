# cosmic-ext-nightlight

[![CI](https://github.com/deepthinker2001/cosmic-ext-nightlight/actions/workflows/ci.yml/badge.svg)](https://github.com/deepthinker2001/cosmic-ext-nightlight/actions/workflows/ci.yml)

A COSMIC panel applet that toggles a night light (a warmer, lower-blue screen
tint) with one click.

## What it does

- Adds a panel button. Clicking it turns night light on or off.
- When on, the screen gamma is set to a fixed warm tint of about 3500 K.
  The colour temperature is not adjustable yet.
- The on/off state is saved and restored the next time the applet starts.
- Gamma is set directly through the Wayland protocol; no other daemon is needed.

## Requirements

- A Wayland compositor that supports `wlr-gamma-control`
  (`zwlr_gamma_control_manager_v1`). COSMIC's compositor, `cosmic-comp`,
  provides it. If the compositor lacks it, the icon stays off and the error is
  printed to stderr.
- COSMIC panel (the applet runs inside it).
- Build tools: a recent stable Rust toolchain, [`just`](https://github.com/casey/just),
  and the system development libraries libcosmic needs. The exact list depends
  on your distribution; see the
  [libcosmic repository](https://github.com/pop-os/libcosmic) for its current
  build dependencies, and install whatever the build reports as missing.

## Install from a release (Debian, Ubuntu, Pop!_OS)

1. Download the `.deb`, its `.deb.asc`, `SHA256SUMS`, `SHA256SUMS.asc` and
   `signing-key.asc` from the latest
   [GitHub Release](../../releases/latest) into an empty folder.
2. [Verify the signature](#verify-a-download) before installing.
3. Install: `sudo apt install ./cosmic-ext-nightlight_*.deb`
4. [Add it to the panel](#add-it-to-the-panel).

To build from source instead, read on.

## Build and install

```sh
just build              # cargo build --release
sudo just install       # installs under /usr
```

`just install` copies the binary to `/usr/bin/cosmic-ext-nightlight`, the
desktop entry to `/usr/share/applications/io.github.cosmic_ext.Nightlight.desktop`,
and the on/off icons to `/usr/share/icons/hicolor/scalable/status/`.

The default prefix is `/usr`, which the panel is sure to search. The `justfile`
has a `prefix` variable if you need a different location, but only locations on
the XDG data path will be found by the panel.

To remove it: `sudo just uninstall`.

Other recipes: `just check` runs clippy, `just clean` removes build output.

## Build a .deb package

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
install on. Remove it with `sudo apt remove cosmic-ext-nightlight`.

## Signed releases

Pushing a tag like `v0.1.0` (it must equal the `version` in `Cargo.toml`) runs
`.github/workflows/release.yml`. It builds the `.deb` with `just deb` in a job
that has no secrets, signs it in a separate job, and publishes a GitHub Release
in a third. The release contains:

- `cosmic-ext-nightlight_*.deb`
- `cosmic-ext-nightlight_*.deb.asc` (detached GPG signature)
- `SHA256SUMS` and `SHA256SUMS.asc` (the checksums, clear-signed)
- `signing-key.asc` (the public key, for convenience only)

### Verify a download

The releases are signed with the key whose full fingerprint is:

```
FINGERPRINT-TO-BE-FILLED-IN-BY-THE-MAINTAINER-BEFORE-THE-FIRST-RELEASE
```

The public key is published as `signing-key.asc` in each release. Check its
fingerprint against the one above, and do not trust the key by itself: the copy
in the release is only as good as the release. Once the key is listed on a
keyserver or on the maintainer's own pages, compare against that too.

```sh
gpg --show-keys --fingerprint signing-key.asc     # compare with the above
gpg --dearmor < signing-key.asc > signing-key.gpg
```

Then, in the folder with the downloaded files, verify against that key alone
(`gpgv` fails on a bad signature or any other key; plain `gpg --decrypt` would
still print forged text):

```sh
gpgv --keyring ./signing-key.gpg --output SHA256SUMS.verified SHA256SUMS.asc \
  && sha256sum -c --ignore-missing SHA256SUMS.verified
gpgv --keyring ./signing-key.gpg cosmic-ext-nightlight_*.deb.asc cosmic-ext-nightlight_*.deb
```

The hashes are checked from the verified text, not from the unsigned
`SHA256SUMS`. If `gpgv` fails, do not use `SHA256SUMS.verified` or the `.deb`.

### Create the signing key (maintainers, once)

On a trusted machine:

```sh
gpg --quick-generate-key "cosmic-ext-nightlight releases <you@example.com>" ed25519 sign 2y
gpg --list-keys --with-colons --fingerprint you@example.com   # the "fpr" line is the full fingerprint
```

Set a passphrase when gpg asks for one. gpg also saves a revocation certificate
at `~/.gnupg/openpgp-revocs.d/<FINGERPRINT>.rev`; copy it somewhere offline.

Send the private key straight into the environment secret (after creating the
`release` environment, see below), so it never appears on screen or in
scrollback:

```sh
gpg --armor --export-secret-keys KEYID | gh secret set GPG_PRIVATE_KEY --env release
```

Without `gh`, write it to a protected file, paste it into GitHub's secret form,
and destroy the file:

```sh
(umask 077; gpg --armor --export-secret-keys KEYID > private.asc)
shred -u private.asc
```

Never print the private key to the terminal. Paste the full 40-character fingerprint into the code block under
[Verify a download](#verify-a-download), replacing the placeholder line. The
release workflow refuses to run while that placeholder is still in this file.
Keep a backup of the private key and its revocation certificate offline. The key
expires after two years: extend it or make a new one before then, and update
the fingerprint here if it changes.

### Cut a release (maintainers)

1. Make sure the fingerprint is filled in and CI is green on `main`.
2. Set `version` in `Cargo.toml` (and commit the updated `Cargo.lock`).
3. Tag and push: `git tag v0.1.0 && git push origin v0.1.0` (the tag must be
   `v` plus the `Cargo.toml` version).
4. Approve the run of the `release` environment when GitHub asks.
5. Check the new release has all five files and that you can
   [verify a download](#verify-a-download) from it.

### Secrets (maintainers)

The signing key is never committed. Create a GitHub environment named
`release` (Settings → Environments), limit its deployment branches and tags to
`v*`, add a required reviewer, and add these as **environment secrets** of that
environment (not repository secrets, which every workflow run can read):

| Secret | Contents |
| --- | --- |
| `GPG_PRIVATE_KEY` | ASCII-armoured private signing key (export it as in [Create the signing key](#create-the-signing-key-maintainers-once); never print it) |
| `GPG_KEY_ID` | Key ID or fingerprint to sign with |
| `GPG_PASSPHRASE` | The key's passphrase (leave empty if it has none) |

Also add a tag ruleset protecting `v*` (restrict who can create, update and
delete those tags). The build tools are installed at fixed versions (`just`
1.59.0, `cargo-deb` 3.8.0) in the workflow; bump them by hand in a reviewed
change.

## Add it to the panel

Open COSMIC Settings, go to the panel (or dock) settings, and use its applet
configuration to add **Night Light** to the start, center or end of the panel.
The exact menu names may differ between COSMIC versions.

If it does not appear, log out and back in so the panel reloads its applet list.

## License

GPL-3.0-only.
