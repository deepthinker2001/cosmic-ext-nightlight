#!/usr/bin/env python3
"""Write THIRD-PARTY-NOTICES: the license text of every crate in the Linux build.

Usage (needs the crates fetched, e.g. after `cargo fetch --locked`):
    python3 -I scripts/gen-third-party-notices.py > THIRD-PARTY-NOTICES

Reads `cargo metadata --locked` for x86_64-unknown-linux-gnu, so the output
depends only on Cargo.lock. Crates are sorted by name and version, so the
output is the same on every run. Crates with no license file in their
package get their SPDX expression and repository instead.
"""
import json
import os
import re
import subprocess
import sys

LICENSE_FILE = re.compile(r"^(LICEN[CS]E|COPYING|NOTICE|UNLICENSE)", re.I)


def license_files(folder):
    found = []
    for name in sorted(os.listdir(folder)):
        path = os.path.join(folder, name)
        if LICENSE_FILE.match(name) and os.path.isfile(path):
            with open(path, encoding="utf-8", errors="replace") as f:
                found.append((name, f.read().strip()))
    return found


def main():
    out = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1",
         "--filter-platform", "x86_64-unknown-linux-gnu"],
        check=True, capture_output=True, text=True).stdout
    meta = json.loads(out)
    root = meta["resolve"]["root"]
    packages = sorted(meta["packages"], key=lambda p: (p["name"], p["version"]))
    print("THIRD-PARTY NOTICES for cosmic-ext-nightlight\n")
    print("cosmic-ext-nightlight itself is GPL-3.0-only (see LICENSE). It is built")
    print("from the crates below, whose license notices follow. Fonts and other")
    print("bundled assets are listed in docs/BUNDLED-ASSETS.md.\n")
    print("Crates marked \"no license field in Cargo.toml\" come from the pop-os")
    print("libcosmic and cosmic-panel git repositories. The license text printed")
    print("for them is the one at the root of that repository: MPL-2.0 for")
    print("libcosmic and its sub-crates, GPL-3.0 for cosmic-panel-config and")
    print("xdg-shell-wrapper-config.\n")
    # Crates in the same package often share one license text; print each
    # distinct text once, with the crates it covers.
    texts = {}
    for p in packages:
        if p["id"] == root:
            continue
        folder = os.path.dirname(p["manifest_path"])
        found = license_files(folder)
        if not found and "/git/checkouts/" in folder:
            # A crate in a git workspace keeps its license at the workspace
            # root (at most three folders up, inside the checkout).
            up = folder
            for _ in range(3):
                up = os.path.dirname(up)
                found = license_files(up)
                if found:
                    break
        header = "{} {} ({})".format(
            p["name"], p["version"], p["license"] or "no license field in Cargo.toml")
        if p.get("repository"):
            header += " <{}>".format(p["repository"])
        if not found:
            found = [("", "No license file is shipped in this crate's package; "
                          "see the repository above.")]
        for name, text in found:
            texts.setdefault(text, []).append(header)
    for text, headers in sorted(texts.items(), key=lambda kv: kv[1][0]):
        print("=" * 78)
        for h in headers:
            print(h)
        print("-" * 78)
        print(text)
        print()
    print("Crates covered: {}".format(len(packages) - 1), file=sys.stderr)


if __name__ == "__main__":
    main()
