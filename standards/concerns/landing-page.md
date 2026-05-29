# Website with Standards

Each tool has a landing page with consistent aesthetic and content.

## Why

The tools are public. People (and agents) discover them via their websites. A consistent design language across all tools signals that they're a cohesive suite, not unrelated projects. Install instructions must be correct and current. Ecosystem links help users discover sibling tools.

## What compliance looks like

- `docs/index.html` exists
- `docs/CNAME` exists with the tool's custom domain
- Dark/light mode support
- Clean typography matching the established trunc/tmux-bridge aesthetic
- Required content: tool name, description, install command, repo link, site link
- Ecosystem footer linking all sibling tools and the umbrella site
- Install commands are current (point to latest release URL)

## How to bring a tool into compliance

1. Copy the aesthetic from an existing compliant tool's site (trunc or tmux-bridge)
2. Update content for this tool
3. Verify dark/light mode works
4. Verify all links resolve
5. Verify install commands actually work (download the binary, run it)

## How to maintain

When a new tool is added to the suite: update the ecosystem footer on ALL tool sites. When release URLs or install methods change: update all sites that reference them.
