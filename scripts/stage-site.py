#!/usr/bin/env python3
"""Stage deterministic site sources and add deployment-only build metadata."""

import argparse
import json
import shutil
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--git-commit", required=True)
    parser.add_argument("--built-at", required=True)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.output.exists():
        shutil.rmtree(args.output)
    shutil.copytree(args.source, args.output)

    version_path = args.output / "version.json"
    with version_path.open() as handle:
        version = json.load(handle)
    version["git_commit"] = args.git_commit
    version_path.write_text(json.dumps(version, indent=2) + "\n")


if __name__ == "__main__":
    main()
