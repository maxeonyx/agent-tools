#!/usr/bin/env python3
"""Generate the umbrella docs/version.json from tool version artifacts."""

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TOOLS_RS = ROOT / "crates/standards/src/lib.rs"
OUTPUT = ROOT / "docs/version.json"


def tool_names() -> list[str]:
    source = TOOLS_RS.read_text()
    match = re.search(r"pub const TOOLS:\s*&\[&str\]\s*=\s*&\[(.*?)\];", source, re.S)
    if not match:
        raise SystemExit(f"could not find TOOLS list in {TOOLS_RS}")
    return re.findall(r'"([^"]+)"', match.group(1))


def tool_version(tool: str) -> str:
    path = ROOT / "tools" / tool / "docs/version.json"
    with path.open() as handle:
        version = json.load(handle)["version"]
    return version


def main() -> None:
    data = {
        "site": "agent-tools",
        "tools": {tool: tool_version(tool) for tool in tool_names()},
    }
    OUTPUT.write_text(json.dumps(data, indent=2) + "\n")


if __name__ == "__main__":
    main()
