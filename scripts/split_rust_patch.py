#!/usr/bin/env python3
"""Split patches/rust/thingos-pal.patch into numbered logical groups.

Groups are determined by which subsystem each changed file belongs to.
Run from the repository root:

    python3 scripts/split_rust_patch.py

Output files are written to patches/rust/ with a NN-name.patch naming
convention so just rust-apply-patches applies them in the correct order.
The original thingos-pal.patch is left untouched so it can be reviewed
before deletion.
"""

import re
import sys
from pathlib import Path

# Mapping: (prefix, short-name) for each group.
# Files not matched fall into the last catch-all group.
GROUPS = [
    ("00-core-prelude",  re.compile(r"^library/core/")),
    ("10-std-glue",      re.compile(r"^library/std/(Cargo\.toml|src/lib\.rs|src/prelude/|src/sys/env_consts)")),
    ("20-pal",           re.compile(r"^library/std/src/sys/pal/")),
    ("30-io-error",      re.compile(r"^library/std/src/sys/io/")),
    ("40-alloc",         re.compile(r"^library/std/src/sys/alloc/")),
    ("50-stdio",         re.compile(r"^library/std/src/sys/stdio/")),
    ("60-pipe",          re.compile(r"^library/std/src/sys/pipe/")),
    ("70-fs",            re.compile(r"^library/std/src/sys/fs/")),
    ("80-process",       re.compile(r"^library/std/src/sys/process/")),
    ("85-thread",        re.compile(r"^library/std/src/sys/thread")),
    ("90-services",      re.compile(r"^library/std/src/sys/(args|env|random|time)/")),
    ("95-net",           re.compile(r"^library/std/src/sys/net/")),
    ("99-misc",          re.compile(r".*")),  # catch-all
]


def split_hunks(patch_text: str) -> list[tuple[str, str]]:
    """Return list of (filepath, hunk_text) for each diff block in the patch."""
    blocks: list[tuple[str, str]] = []
    current_file: str | None = None
    current_lines: list[str] = []

    for line in patch_text.splitlines(keepends=True):
        m = re.match(r"^diff --git a/(.+) b/", line)
        if m:
            if current_file is not None:
                blocks.append((current_file, "".join(current_lines)))
            current_file = m.group(1)
            current_lines = [line]
        else:
            if current_file is not None:
                current_lines.append(line)

    if current_file is not None:
        blocks.append((current_file, "".join(current_lines)))

    return blocks


def group_for(filepath: str) -> str:
    for name, pattern in GROUPS:
        if pattern.match(filepath):
            return name
    return "99-misc"


def main(argv: list[str]) -> int:
    root = Path(__file__).parent.parent
    src = root / "patches" / "rust" / "thingos-pal.patch"
    out_dir = root / "patches" / "rust"

    if not src.exists():
        print(f"ERROR: {src} not found", file=sys.stderr)
        return 1

    patch_text = src.read_text()
    blocks = split_hunks(patch_text)

    if not blocks:
        print("No diff blocks found in patch.", file=sys.stderr)
        return 1

    # Bucket by group
    buckets: dict[str, list[str]] = {}
    for filepath, hunk in blocks:
        g = group_for(filepath)
        buckets.setdefault(g, []).append(hunk)

    print(f"Found {len(blocks)} diff blocks across {len(buckets)} groups:")
    for name in sorted(buckets):
        count = len(buckets[name])
        out_path = out_dir / f"{name}.patch"
        content = "".join(buckets[name])
        out_path.write_text(content)
        print(f"  {out_path.name:40s}  ({count} file(s))")

    print()
    print("Done. Review the new files, then remove thingos-pal.patch once satisfied.")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
