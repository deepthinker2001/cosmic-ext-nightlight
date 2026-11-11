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
  and the development libraries libcosmic needs (for example `libxkbcommon`,
  `libwayland`, `libinput`, `libudev`, `libssl`, `pkg-config`). Check libcosmic's
  documentation if the build reports a missing library.

## Build and install

```sh
just build              # cargo build --release
sudo just install       # installs under /usr
```

`just install` copies the binary to `/usr/bin/cosmic-ext-nightlight`, the
desktop entry to `/usr/share/applications/io.github.cosmic_ext.Nightlight.desktop`,
and the on/off icons to `/usr/share/icons/hicolor/scalable/status/`.

To install somewhere else, override the prefix, e.g.
`just prefix=$HOME/.local install` (the panel only finds applets in locations
on the XDG data path, so `/usr` is the simplest choice).

To remove it: `sudo just uninstall`.

Other recipes: `just check` runs clippy, `just clean` removes build output.

## Add it to the panel

1. Open **Settings → Desktop → Panel** (or **Dock**).
2. Choose **Configure panel applets** (or **Configure dock applets**).
3. Find **Night Light** in the applet list and add it to the start, center or
   end of the panel.

If it does not appear, restart the panel (log out and back in, or run
`pkill cosmic-panel`; the session restarts it).

## License

GPL-3.0-only.
