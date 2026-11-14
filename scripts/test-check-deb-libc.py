#!/usr/bin/env python3
"""Integration tests using actual Debian control metadata, no Rust build needed."""

import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("check-deb-libc.py")


class LibcFloorTests(unittest.TestCase):
    def test_dependency_floors(self):
        cases = [
            ("libc6 (>= 2.39), cosmic-panel", True),
            ("cosmic-panel, libc6 (>= 2.38), libxkbcommon0 (>= 1.0)", True),
            ("libc6:amd64 (>= 2.9)", True),
            ("libc6 (>= 2.43), cosmic-panel", False),
            ("libc6 (>= 2.40)", False),
            ("libc6 (>= 2.39.1)", False),
            ("cosmic-panel", False),
            ("libc6", False),
            ("libc6 (>> 2.39)", False),
            ("libc6 (>= 2.39) | other-libc", False),
            ("libc6 (>= 2.39), libc6 (>= 2.43)", False),
        ]
        with tempfile.TemporaryDirectory(dir=os.environ.get("FERRIVIDE_SCRATCH")) as tmp:
            root = Path(tmp)
            control = root / "pkg" / "DEBIAN"
            control.mkdir(parents=True)
            deb = root / "test package.deb"
            for depends, accepted in cases:
                with self.subTest(depends=depends):
                    (control / "control").write_text(
                        "Package: libc-floor-test\nVersion: 1.0\nArchitecture: amd64\n"
                        "Maintainer: Test <test@example.invalid>\nDescription: Test\n"
                        f"Depends: {depends}\n"
                    )
                    subprocess.run(
                        ["dpkg-deb", "--build", str(control.parent), str(deb)],
                        check=True, capture_output=True,
                    )
                    result = subprocess.run(
                        [sys.executable, str(SCRIPT), str(deb)], capture_output=True, text=True
                    )
                    self.assertEqual(result.returncode == 0, accepted, result.stdout + result.stderr)

    def test_missing_package(self):
        result = subprocess.run(
            [sys.executable, str(SCRIPT), "nonexistent.deb"], capture_output=True
        )
        self.assertNotEqual(result.returncode, 0)

    def test_requires_one_package(self):
        for args in ([], ["one.deb", "two.deb"]):
            with self.subTest(args=args):
                result = subprocess.run([sys.executable, str(SCRIPT), *args], capture_output=True)
                self.assertNotEqual(result.returncode, 0)


if __name__ == "__main__":
    unittest.main()
