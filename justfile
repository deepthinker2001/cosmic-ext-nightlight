name := 'cosmic-ext-nightlight'
appid := 'io.github.cosmic_ext.Nightlight'
prefix := '/usr'
bin-dst := prefix / 'bin' / name
desktop-dst := prefix / 'share/applications' / appid + '.desktop'
icon-dir := prefix / 'share/icons/hicolor/scalable/status'

default: build

build:
    cargo build --release

check:
    cargo clippy --all-targets

clean:
    cargo clean

install:
    install -Dm0755 target/release/{{name}} {{bin-dst}}
    install -Dm0644 data/{{appid}}.desktop {{desktop-dst}}
    install -Dm0644 data/icons/{{appid}}-on-symbolic.svg {{icon-dir}}/{{appid}}-on-symbolic.svg
    install -Dm0644 data/icons/{{appid}}-off-symbolic.svg {{icon-dir}}/{{appid}}-off-symbolic.svg

uninstall:
    rm -f {{bin-dst}} {{desktop-dst}} {{icon-dir}}/{{appid}}-on-symbolic.svg {{icon-dir}}/{{appid}}-off-symbolic.svg
