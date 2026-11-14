# cosmic-ext-nightlight

[![CI](https://github.com/deepthinker2001/cosmic-ext-nightlight/actions/workflows/ci.yml/badge.svg)](https://github.com/deepthinker2001/cosmic-ext-nightlight/actions/workflows/ci.yml)

A panel applet for COSMIC that toggles a night light (a warmer, lower-blue screen
tint) with one click.

## What it does

- Adds a panel button. Clicking it turns night light on or off.
- When on, the screen gamma is set to a fixed warm tint of about 3500 K.
  The colour temperature is not adjustable yet.
- The on/off state is saved and restored the next time the applet starts.
- Gamma is set directly through the Wayland protocol; no other daemon is needed.

### Auto (sunrise/sunset)

Right-click the button and switch on **Auto (sunrise/sunset)**. The screen
then warms up smoothly in the evening and cools back down in the morning,
instead of switching at a fixed time:

- Sunrise and sunset for today are calculated on your computer from your
  location (a standard solar-position formula; polar day and night are
  handled: the screen stays neutral all day, or fully warm all day).
- The colour temperature slider sets the target. The ramp goes in 1% steps
  from neutral (6500 K, 0%) to your target (100%) along a smooth sine-squared
  curve. The evening ramp runs 0 to 100% around sunset, the morning ramp
  100 to 0% around sunrise. Each ramp is 60 minutes long, centred on the
  event (30 minutes before to 30 minutes after); the popup says so. The
  length is a constant in `src/solar.rs` (`RAMP_WINDOW_MINUTES`).
- A step is applied only when the whole percent changes, checked every 20
  seconds, so it also catches up after the computer wakes from sleep. Moving
  the slider applies the new target at once. The panel button still switches
  night light on or off by hand; that holds until the next step. Switching
  on by hand uses the current ramp temperature, or your full target when the
  ramp is at 0% (daytime); the next 1% step then takes over.
- **Location**: by default the coordinates of the system timezone, read from
  `/usr/share/zoneinfo/zone.tab` (or `zone1970.tab`). You can type a latitude
  and longitude in the popup instead (they win once both are valid), and
  **Use timezone location** goes back to the default. If neither works, the
  popup shows a note and Auto stays off. There is no IP lookup and no network
  access.
- Auto and the custom schedule exclude each other: switching one on switches
  the other off.
- The settings are saved with the others.

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

## Install (Debian, Ubuntu, Pop!_OS)

### With apt (recommended)

Add the signed repository once; updates then arrive with your normal system
updates. First download the key to a temporary file and check its fingerprint
before you trust it:

```sh
key=$(mktemp)
curl -fsSL https://deepthinker2001.github.io/cosmic-ext-nightlight/key.gpg -o "$key"
gpg --show-keys --with-fingerprint "$key"
```

The fingerprint printed must be exactly:

```text
0472 CEF0 1CE0 E79E C118  662D 34A0 6EB9 79E5 8584
```

If it differs, run `rm "$key"` and stop. If it matches, trust the key and
add the repository (in the same terminal, so `$key` is still set):

```sh
sudo gpg --dearmor -o /usr/share/keyrings/cosmic-ext-nightlight.gpg "$key"
rm "$key"
echo "deb [signed-by=/usr/share/keyrings/cosmic-ext-nightlight.gpg] https://deepthinker2001.github.io/cosmic-ext-nightlight stable main" \
  | sudo tee /etc/apt/sources.list.d/cosmic-ext-nightlight.list
sudo apt update
sudo apt install cosmic-ext-nightlight
```

Then [add it to the panel](#add-it-to-the-panel).

To update to a new version: `sudo apt update && sudo apt upgrade`, or just
install your system updates as usual.

Remove it with `sudo apt remove cosmic-ext-nightlight`. To also stop trusting
the repository, delete `/etc/apt/sources.list.d/cosmic-ext-nightlight.list`
and `/usr/share/keyrings/cosmic-ext-nightlight.gpg`.

### From a release

Download the `.deb` (and its `.sha256`, to check the download) from the
[releases page](../../releases/latest), then:

```sh
sha256sum -c cosmic-ext-nightlight_*.deb.sha256
sudo apt install ./cosmic-ext-nightlight_*.deb
```

This does not update itself; use the apt repository above for that.

To build from source instead, see
[Build and install from source](#build-and-install-from-source) below.

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
