#!/usr/bin/env python3
"""
Compliance checker for agent-tools workspace.

Reads standards/concerns.toml and standards/compliance.toml,
verifies what it can mechanically, and prints a matrix.

Exit code 0: all checks pass (non-compliant is OK if honestly declared).
Exit code 1: inconsistency (missing status, undeclared tool, checker disagrees with claimed status).
"""

import sys
import tomllib
from pathlib import Path

WORKSPACE_ROOT = Path(__file__).resolve().parent.parent
TOOLS_DIR = WORKSPACE_ROOT / "tools"
CONCERNS_FILE = WORKSPACE_ROOT / "standards" / "concerns.toml"
COMPLIANCE_FILE = WORKSPACE_ROOT / "standards" / "compliance.toml"

TOOLS = ["trunc", "tb", "dotsync", "tdd-ratchet", "oc"]
VALID_STATUSES = {"compliant", "waived", "non-compliant", "not-applicable"}


def load_concerns():
    with open(CONCERNS_FILE, "rb") as f:
        data = tomllib.load(f)
    return data.get("concerns", {})


def load_compliance():
    with open(COMPLIANCE_FILE, "rb") as f:
        data = tomllib.load(f)
    # Filter out non-tool sections
    return {k: v for k, v in data.items() if k in TOOLS}


def check_workspace_routing(tool_path: Path) -> bool:
    """Check if tool AGENTS.md mentions developing from agent-tools workspace."""
    agents_file = tool_path / "AGENTS.md"
    if not agents_file.exists():
        return False
    content = agents_file.read_text()
    return "agent-tools" in content.lower() and "workspace" in content.lower()


def check_tdd_ratchet(tool_path: Path) -> bool:
    """Check if .test-status.json exists (basic indicator of ratchet use)."""
    return (tool_path / ".test-status.json").exists()


def check_opencode_skill(tool_path: Path) -> bool:
    """Check if docs/SKILL.md exists."""
    return (tool_path / "docs" / "SKILL.md").exists()


def check_landing_page(tool_path: Path) -> bool:
    """Check if docs/index.html exists."""
    return (tool_path / "docs" / "index.html").exists()


def check_version_bump_guard(tool_path: Path) -> bool:
    """Check if CI workflow mentions version bump enforcement."""
    ci_file = tool_path / ".github" / "workflows" / "ci.yml"
    if not ci_file.exists():
        return False
    content = ci_file.read_text()
    return "version" in content.lower() and ("bump" in content.lower() or "tag" in content.lower())


CHECKERS = {
    "workspace-routing": check_workspace_routing,
    "tdd-ratchet": check_tdd_ratchet,
    "opencode-skill": check_opencode_skill,
    "landing-page": check_landing_page,
    "version-bump-guard": check_version_bump_guard,
    # auto-update and shared-ci-pattern: no checker yet (aspirational concerns)
}


def main():
    concerns = load_concerns()
    compliance = load_compliance()
    errors = []

    # Check completeness: every tool must have a status for every concern
    for tool in TOOLS:
        if tool not in compliance:
            errors.append(f"Tool '{tool}' missing from compliance.toml")
            continue
        for concern_id in concerns:
            if concern_id not in compliance[tool]:
                errors.append(f"Tool '{tool}' missing status for concern '{concern_id}'")
            elif compliance[tool][concern_id] not in VALID_STATUSES:
                errors.append(
                    f"Tool '{tool}' has invalid status '{compliance[tool][concern_id]}' "
                    f"for concern '{concern_id}'"
                )

    # Run mechanical checks where possible
    verification_failures = []
    for tool in TOOLS:
        if tool not in compliance:
            continue
        tool_path = TOOLS_DIR / tool
        if not tool_path.exists():
            errors.append(f"Tool directory 'tools/{tool}' does not exist")
            continue

        for concern_id, checker_fn in CHECKERS.items():
            if concern_id not in compliance[tool]:
                continue
            status = compliance[tool][concern_id]
            if status == "not-applicable":
                continue

            actual = checker_fn(tool_path)

            # If claimed compliant but checker disagrees, that's an error
            if status == "compliant" and not actual:
                verification_failures.append(
                    f"Tool '{tool}' claims compliant for '{concern_id}' but checker says NO"
                )
            # If claimed non-compliant but checker says yes, that's a nudge (not error)
            # (tool may have been fixed but compliance.toml not updated)
            if status == "non-compliant" and actual:
                print(
                    f"  NOTE: '{tool}' may now be compliant for '{concern_id}' "
                    f"— consider updating compliance.toml"
                )

    # Print matrix
    print()
    print("=" * 70)
    print("COMPLIANCE MATRIX")
    print("=" * 70)
    print()

    concern_ids = list(concerns.keys())
    header = f"{'Concern':<25}" + "".join(f"{t:<14}" for t in TOOLS)
    print(header)
    print("-" * len(header))

    for concern_id in concern_ids:
        row = f"{concern_id:<25}"
        for tool in TOOLS:
            if tool not in compliance:
                row += f"{'???':<14}"
                continue
            status = compliance[tool].get(concern_id, "???")
            symbol = {
                "compliant": "OK",
                "waived": "WAIVED",
                "non-compliant": "--",
                "not-applicable": "n/a",
            }.get(status, "???")
            row += f"{symbol:<14}"
        print(row)

    print()

    # Report
    if errors:
        print("ERRORS (completeness/validity):")
        for e in errors:
            print(f"  {e}")
        print()

    if verification_failures:
        print("VERIFICATION FAILURES (claimed compliant but checker disagrees):")
        for f in verification_failures:
            print(f"  {f}")
        print()

    if errors or verification_failures:
        print("FAILED")
        return 1

    print("PASSED (all statuses declared, no false compliance claims)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
