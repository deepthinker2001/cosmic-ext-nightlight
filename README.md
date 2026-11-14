# cosmic-ext-nightlight

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

Import `signing-key.asc` (`gpg --import signing-key.asc`) and check that the
fingerprint `gpg --fingerprint` shows matches the one above. Do not trust the
key by itself: the copy in the release is only as good as the release. Then,
in the folder with the downloaded files:

```sh
gpg --decrypt SHA256SUMS.asc | sha256sum -c --ignore-missing -
gpg --verify cosmic-ext-nightlight_*.deb.asc cosmic-ext-nightlight_*.deb
```

The first command checks the hashes from the signed text itself, not from the
unsigned `SHA256SUMS` file. Check that `gpg` reports a good signature from the
key with the fingerprint above.

### Secrets (maintainers)

The signing key is never committed. Create a GitHub environment named
`release` (Settings → Environments), limit its deployment branches and tags to
`v*`, add a required reviewer, and add these as **environment secrets** of that
environment (not repository secrets, which every workflow run can read):

| Secret | Contents |
| --- | --- |
| `GPG_PRIVATE_KEY` | ASCII-armoured private signing key (`gpg --armor --export-secret-keys KEYID`) |
| `GPG_KEY_ID` | Key ID or fingerprint to sign with |
| `GPG_PASSPHRASE` | The key's passphrase (leave empty if it has none) |

Also add a tag ruleset protecting `v*` (restrict who can create, update and
delete those tags), and put the key's fingerprint in this README. The build
tools are installed at fixed versions (`just` 1.59.0, `cargo-deb` 3.8.0) in the
workflow; bump them by hand in a reviewed change.

## Add it to the panel

Open COSMIC Settings, go to the panel (or dock) settings, and use its applet
configuration to add **Night Light** to the start, center or end of the panel.
The exact menu names may differ between COSMIC versions.

If it does not appear, log out and back in so the panel reloads its applet list.

## License

GPL-3.0-only.
