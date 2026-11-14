#!/usr/bin/env python3
"""Reject packages whose auto-detected libc6 floor exceeds Ubuntu 24.04."""

import re
import subprocess
import sys


def check(deb):
    depends = subprocess.check_output(
        ["dpkg-deb", "-f", deb, "Depends"], text=True
    ).strip()
    print(f"{deb}: Depends: {depends}")
    # cargo-deb emits one direct, versioned libc6 dependency. Fail closed if
    # that changes, rather than silently accepting missing/ambiguous metadata.
    entries = [entry.strip() for entry in depends.split(",")]
    libc = [entry for entry in entries if re.search(r"\blibc6\b", entry)]
    if len(libc) != 1:
        raise ValueError("Expected exactly one versioned libc6 dependency")
    match = re.fullmatch(r"libc6(?::[a-z0-9-]+)?\s*\(>=\s*([^\s()]+)\)", libc[0])
    if not match:
        raise ValueError(f"Unexpected libc6 dependency: {libc[0]}")
    floor = match[1]
    # Debian versions are not floats or lexically ordered strings.
    subprocess.run(["dpkg", "--validate-version", floor], check=True)
    if subprocess.run(["dpkg", "--compare-versions", floor, "le", "2.39"]).returncode:
        raise ValueError(f"libc6 floor {floor} exceeds 2.39; build on Ubuntu 24.04")


if __name__ == "__main__":
    try:
        if len(sys.argv) != 2:
            raise ValueError("Expected exactly one .deb path")
        check(sys.argv[1])
    except (ValueError, subprocess.CalledProcessError, OSError) as error:
        sys.exit(str(error))
