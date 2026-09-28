#!/usr/bin/env python3
"""Generate the umbrella docs/version.json from tool version artifacts."""

import json
import os
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "docs/version.json"


# Every released tool the umbrella pins, archived ones included.
TOOLS = ["trunc", "tb", "dotsync", "tdd-ratchet", "oc", "agent-harness"]


def existing_versions() -> dict[str, str]:
    if not OUTPUT.exists():
        return {}
    with OUTPUT.open() as handle:
        return json.load(handle).get("tools", {})


def tool_version(tool: str, existing: dict[str, str]) -> str:
    path = ROOT / "tools" / tool / "docs/version.json"
    if not path.exists():
        if os.environ.get("CI"):
            raise SystemExit(f"CI requires initialized tool version artifact: {path}")
        if tool not in existing:
            raise SystemExit(
                f"uninitialized tool {tool} has no preserved version in {OUTPUT}"
            )
        return existing[tool]
    with path.open() as handle:
        version = json.load(handle)["version"]
    return version


def main() -> None:
    existing = existing_versions()
    data = {
        "site": "agent-tools",
        "tools": {tool: tool_version(tool, existing) for tool in TOOLS},
    }
    OUTPUT.write_text(json.dumps(data, indent=2) + "\n")


if __name__ == "__main__":
    main()
