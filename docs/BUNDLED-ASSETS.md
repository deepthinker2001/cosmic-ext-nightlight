# Bundled assets and third-party notices

What the `cosmic-ext-nightlight` binary and package contain that did not
originate in this project, who owns it, its license, and where the notice is.
The package installs the notices under
`/usr/share/doc/cosmic-ext-nightlight/`.

The project itself is GPL-3.0-only (`LICENSE`, installed there too).

| Asset | Owner | License | Notice |
| --- | --- | --- | --- |
| Open Sans Light, Regular, Semibold, Bold, ExtraBold (compiled in by libcosmic, `EMBEDDED_FONTS`) | The Open Sans Project Authors, <https://github.com/googlefonts/opensans> | SIL OFL 1.1 | `fonts/OFL-OpenSans.txt` (from `res/open-sans/LICENSE` in libcosmic) |
| Noto Sans Mono Regular, Bold (compiled in by libcosmic) | The Noto Project Authors | SIL OFL 1.1 | `fonts/OFL-NotoSansMono.txt` (from `res/noto/LICENSE` in libcosmic) |
| `Iced-Icons.ttf` (compiled in by `iced_graphics` 0.14.0) | iced contributors; the font was generated with Fontello (its metadata says "Copyright (C) 2020 by original authors @ fontello.com") | Shipped inside the MIT-licensed `iced_graphics` crate; no separate license file | The `iced_graphics` MIT text in `THIRD-PARTY-NOTICES`. **Open point:** the upstream license of the individual glyphs was not confirmed (no network access when this was written). Confirm with iced-rs/iced before the first release. |
| FiraSans (iced's `fira-sans` feature) | | | Not enabled in this build; not shipped. |
| `data/icons/*-symbolic.svg` | This project | GPL-3.0-only | Drawn by hand for this project; no third-party source. |
| About 510 Rust crates linked or built for the Linux target | Their authors | Mostly MIT and/or Apache-2.0, plus BSD, ISC, Zlib, Unicode-3.0, MPL-2.0, CC0, BSL-1.0 | `THIRD-PARTY-NOTICES` |

Note: libcosmic's `applet` feature also links GPL-3.0 crates from
pop-os/cosmic-panel (`cosmic-panel-config`, `xdg-shell-wrapper-config`), and
libcosmic itself is MPL-2.0; both are in `THIRD-PARTY-NOTICES`.

## Regenerating `THIRD-PARTY-NOTICES`

The file is generated from `Cargo.lock` by `scripts/gen-third-party-notices.py`
(Python 3 standard library only; the crates must be fetched):

```sh
just notices        # cargo fetch --locked, then rewrites THIRD-PARTY-NOTICES
```

Regenerate it whenever `Cargo.lock` changes, and commit the result. Crates
that ship no license file are listed with their declared license and
repository only; check those by hand if a new one appears.
