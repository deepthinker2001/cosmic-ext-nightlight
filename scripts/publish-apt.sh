#!/bin/sh
# Rebuild the signed apt repository that GitHub Pages serves.
#
# Usage: scripts/publish-apt.sh <site-dir> <key-id> [new .deb files...]
#
# <site-dir> is a checkout of the gh-pages branch. New .deb files are added
# to its pool (earlier ones stay, so apt can still find them); the package
# index and the signed Release files are then generated again from the whole
# pool. Signing uses the key <key-id> from the keyring in $GNUPGHOME.
# Needs apt-ftparchive (apt-utils) and gpg.
set -eu

if [ $# -lt 2 ]; then
    echo "usage: $0 <site-dir> <key-id> [new .deb files...]" >&2
    exit 2
fi
site=$1
key=$2
shift 2

pool=pool/main/c/cosmic-ext-nightlight
dist=dists/stable
mkdir -p "$site/$pool" "$site/$dist/main/binary-amd64"
for deb in "$@"; do
    cp "$deb" "$site/$pool/"
done

cd "$site"
apt-ftparchive packages pool > "$dist/main/binary-amd64/Packages"
gzip -9 -n -k -f "$dist/main/binary-amd64/Packages"

# Written beside the dists tree first: apt-ftparchive would otherwise list
# the half-written Release file in itself.
apt-ftparchive \
    -o APT::FTPArchive::Release::Origin=deepthinker2001 \
    -o APT::FTPArchive::Release::Label="Night Light for COSMIC" \
    -o APT::FTPArchive::Release::Suite=stable \
    -o APT::FTPArchive::Release::Codename=stable \
    -o APT::FTPArchive::Release::Architectures=amd64 \
    -o APT::FTPArchive::Release::Components=main \
    -o APT::FTPArchive::Release::Description="Night Light for COSMIC: a panel applet that toggles a night light" \
    release "$dist" > Release.new
mv Release.new "$dist/Release"

gpg --batch --yes --local-user "$key" --clearsign -o "$dist/InRelease" "$dist/Release"
gpg --batch --yes --local-user "$key" --armor --detach-sign -o "$dist/Release.gpg" "$dist/Release"
gpg --batch --yes --armor --export "$key" > key.gpg

# Check that the public key users download is the one asked for and that it
# verifies both signatures, so a wrong key or a bad signature fails here
# instead of on every user's `apt update`.
want=$(printf '%s' "$key" | tr -d ' ' | tr '[:lower:]' '[:upper:]')
got=$(gpg --batch --with-colons --import-options show-only --import key.gpg \
    | awk -F: '$1 == "fpr" { print $10; exit }')
if [ "$got" != "$want" ]; then
    echo "key.gpg has fingerprint '$got', not '$want' (pass the full fingerprint)." >&2
    exit 1
fi
check=$(mktemp -d)
trap 'rm -rf "$check"' EXIT
gpg --batch --dearmor -o "$check/key.gpg" key.gpg
gpgv --keyring "$check/key.gpg" "$dist/InRelease"
gpgv --keyring "$check/key.gpg" "$dist/Release.gpg" "$dist/Release"

# Serve the files as they are, without GitHub Pages' Jekyll processing.
touch .nojekyll
echo "Repository in $site rebuilt and signed with $key."
