# Changelog

What changed in each release, for users. Maintainers: see
[docs/RELEASING.md](docs/RELEASING.md) for how to cut one.

## Unreleased

- Apt setup instructions now specify `arch=amd64` to avoid the harmless
  missing-i386 notice on multiarch systems; the README explains how existing
  users can update their sources line.
- Night light can turn on from the panel again: the applet now asks
  cosmic-panel for the host Wayland display (`X-HostWaylandDisplay`), which
  its gamma connection needs.
- The applet no longer lags: settings are saved in the background instead of
  on every slider step, a left click toggles on press, and toggles and
  slider moves made while a change is being applied are kept and shown at
  once instead of being ignored.

## 0.2.1 - 2026-11-14

### Packaging

- The `.deb` installs on Ubuntu 24.04 again. The 0.2.0 package was built on a
  newer Ubuntu and required `libc6 (>= 2.43)`, so apt refused it on 24.04
  (glibc 2.39). Releases are now built on Ubuntu 24.04, and the release fails
  if the package asks for a libc6 newer than 2.39. No change to the applet
  itself.

## 0.2.0 - 2026-11-13

Everything since 0.1.0. The `v0.1.1` tag points into this work, but its
release build failed and published no package, so these changes ship in this
version.

### Added

- Settings popup: right-click the panel button to open it (right-click
  again to close it). It has the same on/off switch as a left click.
- Colour temperature slider in the popup, from 1900 K to 6500 K in steps of
  100 K (default 3500 K). The change applies at once while night light is
  on.
- Custom schedule: night light turns on at a start time and off at an end
  time you pick (defaults 21:00 and 07:00; the window may cross midnight).
  A failed switch is retried on the next check, 20 seconds later.
- Auto (sunrise/sunset): the screen warms smoothly over the hour around
  sunset and cools over the hour around sunrise. Sunrise and sunset are
  calculated on your computer from your system timezone's location, or from
  a latitude and longitude you type in. There is no network access. Auto and
  the custom schedule exclude each other.
- All settings are saved and restored with the on/off state.

### Changed

- License: the project is GPL-3.0-only, matching `Cargo.toml` and the package metadata. The `LICENSE` file held the AGPL-3.0 text by mistake and is now the GPL-3.0 text. libcosmic's applet support links GPL-3.0-only cosmic-panel crates, so GPL-3.0-only is the compatible choice.

- Described as a "panel applet for COSMIC" (package description included),
  in place of "COSMIC panel applet".
- README: documents the settings, and lists what hasn't been checked on a
  live COSMIC session.

### Packaging

- `scripts/publish-apt.sh` now checks that the published `key.gpg` has the
  expected fingerprint and verifies the repository's signatures with it
  before the repository is pushed.
- The package installs third-party notices under
  `/usr/share/doc/cosmic-ext-nightlight/`: `THIRD-PARTY-NOTICES` for the
  Rust crates, `BUNDLED-ASSETS.md`, and the font licenses in `fonts/`
  (Open Sans, Noto Sans Mono, and Font Awesome, whose glyphs make up most of
  iced's built-in icon font).

## 0.1.0 - 2026-11-13

First release.

- Panel button that turns night light on or off with one click, at a fixed
  warm tint of about 3500 K.
- Sets the screen gamma directly through the Wayland `wlr-gamma-control`
  protocol; no other daemon is needed.
- Remembers the on/off state between starts.
- `.deb` package, attached to the GitHub release. The signed apt repository
  was not published: the release's signing job was rejected by the GitHub
  environment's protection rules.
