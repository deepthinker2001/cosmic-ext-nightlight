#!/bin/sh
# Rebuild the signed apt repository that GitHub Pages serves.
#
# Usage: scripts/publish-apt.sh <site-dir> <key-id> [new .deb files...]
#
# <site-dir> is a checkout of the gh-pages branch. New .deb files are added
# to its pool (earlier ones stay, so apt can still find them); the package
# index and the signed Release files are then generated again from the whole
# pool. Signing uses the key <key-id> from the keyring in $GNUPGHOME; it must
# be the key's full fingerprint (spaces and letter case don't matter).
# Needs apt-ftparchive (apt-utils), gpg and gpgv.
set -eu

if [ $# -lt 2 ]; then
    echo "usage: $0 <site-dir> <key-id: full fingerprint> [new .deb files...]" >&2
    exit 2
fi
site=$1
key=$2
shift 2

# Check the key before anything is signed: <key-id> must name exactly the
# key in the keyring whose full fingerprint it is.
want=$(printf '%s' "$key" | tr -d ' ' | tr '[:lower:]' '[:upper:]')
got=$(gpg --batch --with-colons --list-secret-keys -- "$want" 2>/dev/null \
    | awk -F: '$1 == "fpr" { print $10; exit }') || true
if [ "$got" != "$want" ]; then
    echo "No secret key with fingerprint '$want' in the keyring (found '$got');" \
        "pass the key's full fingerprint." >&2
    exit 1
fi

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

# Keep these instructions and the pinned fingerprint in sync with README.md.
# Quote the heredoc so the install commands are text, never run by publishing.
cat > index.html <<'HTML'
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Night Light for COSMIC — apt repository</title>
</head>
<body>
  <main>
    <h1>Night Light for COSMIC</h1>
    <p>A COSMIC panel applet that toggles a warmer, lower-blue screen tint
      with one click. This is its signed apt repository for Debian, Ubuntu
      and Pop!_OS (amd64); COSMIC panel is required.</p>
    <p><a href="https://github.com/deepthinker2001/cosmic-ext-nightlight">GitHub repository and documentation</a>
      · <a href="key.gpg">Download the public signing key (key.gpg)</a></p>

    <h2>Install with apt</h2>
    <p>Add the signed repository once; updates then arrive with your normal
      system updates. First download the key to a temporary file and check
      its fingerprint before you trust it:</p>
    <pre><code>key=$(mktemp)
curl -fsSL https://deepthinker2001.github.io/cosmic-ext-nightlight/key.gpg -o "$key"
gpg --show-keys --with-fingerprint "$key"
</code></pre>
    <p>The fingerprint printed must be exactly:</p>
    <pre><code>0472 CEF0 1CE0 E79E C118  662D 34A0 6EB9 79E5 8584</code></pre>
    <p><strong>If it differs, run <code>rm "$key"</code> and stop.</strong>
      If it matches, trust the key and add the repository (in the same
      terminal, so <code>$key</code> is still set):</p>
    <pre><code>sudo gpg --dearmor -o /usr/share/keyrings/cosmic-ext-nightlight.gpg "$key"
rm "$key"
echo "deb [signed-by=/usr/share/keyrings/cosmic-ext-nightlight.gpg] https://deepthinker2001.github.io/cosmic-ext-nightlight stable main" \
  | sudo tee /etc/apt/sources.list.d/cosmic-ext-nightlight.list
sudo apt update
sudo apt install cosmic-ext-nightlight
</code></pre>
    <p>Then follow the
      <a href="https://github.com/deepthinker2001/cosmic-ext-nightlight#add-it-to-the-panel">instructions to add it to the panel</a>.</p>
  </main>
</body>
</html>
HTML

# Serve the files as they are, without GitHub Pages' Jekyll processing.
touch .nojekyll
echo "Repository in $site rebuilt and signed with $key."
