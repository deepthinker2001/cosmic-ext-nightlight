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
- To build from source: a recent stable Rust toolchain,
  [`just`](https://github.com/casey/just), and the system development
  libraries libcosmic needs. The exact list depends on your distribution; see
  the [libcosmic repository](https://github.com/pop-os/libcosmic) for its
  current build dependencies, and install whatever the build reports as missing.

## Install from a release (Debian, Ubuntu, Pop!_OS)

1. Download the `.deb`, its `.deb.asc`, `SHA256SUMS`, `SHA256SUMS.asc` and
   `signing-key.asc` from the latest
   [GitHub Release](../../releases/latest) into an empty folder.
2. [Verify the download](#verify-a-download) before installing.
3. Install: `sudo apt install ./cosmic-ext-nightlight_*.deb`
4. [Add it to the panel](#add-it-to-the-panel).

Remove it with `sudo apt remove cosmic-ext-nightlight`.

To build from source instead, see
[Build and install from source](#build-and-install-from-source) below.

## Verify a download

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

## Build and install from source

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

## Add it to the panel

Open COSMIC Settings, go to the panel (or dock) settings, and use its applet
configuration to add **Night Light** to the start, center or end of the panel.
The exact menu names may differ between COSMIC versions.

If it does not appear, log out and back in so the panel reloads its applet list.

## License

GPL-3.0-only.

Maintainers: see [docs/RELEASING.md](docs/RELEASING.md) for packaging and releases.
