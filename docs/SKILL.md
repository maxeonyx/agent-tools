---
name: agent-tools
description: When creating a new CLI tool for AI agent workflows, or setting up a repo following the maxeonyx agent-tools pattern
---

# agent-tools

Use this pattern for lightweight, docs-first CLI tool repos in the maxeonyx ecosystem.

## Goal

Ship a small, discoverable tool repository with:
- a Pages landing site,
- an OpenCode skill,
- release/deploy workflows,
- and cross-references from sibling tools.

## Repository setup checklist

1. Create the GitHub repository (public unless explicitly private).
2. Add root files:
   - `README.md` (minimal pointer to website)
   - `AGENTS.md` (agent guidance for this repo)
   - `LICENSE` (MIT, Max Clarke, 2026)
3. Add docs files:
   - `docs/index.html` (landing page)
   - `docs/SKILL.md` (tool-specific OpenCode skill)
   - `docs/CNAME` (custom domain)
4. Add `.github/workflows/pages.yml` using Actions-based deployment from `docs/`.

## Landing page requirements

Use the established trunc/tmux-bridge aesthetic:
- support light + dark mode,
- keep typography and spacing clean,
- include install commands and repo/site links.

For each tool entry include:
- tool name,
- binary name,
- one-line description,
- repository URL,
- website URL,
- install command snippet.

Include a small ecosystem footer section linking:
- trunc,
- tmux-bridge,
- dotsync,
- tdd-ratchet,
- oc,
- and umbrella site `https://tools.maxeonyx.com`.

## Workflow templates

### Pages workflow

- Trigger on pushes to `main` affecting `docs/**` or the workflow itself.
- Use:
  - `actions/configure-pages@v5`
  - `actions/upload-pages-artifact@v3`
  - `actions/deploy-pages@v4`
- Upload `docs` as the Pages artifact path.

### Release workflow (for binary tools)

- Trigger on `v*` tags.
- Build cross-platform artifacts (Linux/macOS/Windows).
- Upload artifacts and create a GitHub Release.
- Keep binary naming consistent with tool command.

### Distribution: GitHub releases vs cargo publish

- **End-user tools** (trunc, tmux-bridge, dotsync, oc): distribute as **bare binaries via tool Pages release paths** (`https://<tool>.maxeonyx.com/releases/<asset>`), backed by GitHub Releases.
- **Developer tools** (tdd-ratchet / cargo-ratchet): offer both bare binary download via Pages release path and `cargo install tdd-ratchet`.
- **Release asset naming**: `<binary>-<arch>-<os>` for unix (e.g. `trunc-x86_64-linux`), `<binary>-<arch>-<os>.exe` for windows. No tarballs or zips — bare binaries only.

## OpenCode skill installation

Install repo skill into local opencode config:

```bash
mkdir -p ~/.config/opencode/skills/<tool-name>
cp docs/SKILL.md ~/.config/opencode/skills/<tool-name>/SKILL.md
```

If this is a global reusable pattern skill, use `agent-tools` as skill name.

## DNS and Pages

1. Add `docs/CNAME` with the tool domain.
2. Ensure GitHub Pages source is GitHub Actions.
3. Configure DNS for the domain to point at GitHub Pages.
4. Verify site loads over HTTPS.

## Cross-references

When introducing a new tool, add/update a compact footer reference on each sibling landing page:

`Part of maxeonyx agent-tools: trunc | tb | dotsync | tdd-ratchet | oc`

Link each label to its site and link umbrella text to `https://tools.maxeonyx.com`.

## Verification

- Confirm required files exist.
- Confirm Pages workflow is present and valid YAML.
- Confirm `CNAME` value is exact.
- Confirm skill frontmatter has correct `name` and trigger-focused `description`.
- Confirm landing page links and install commands are correct.
- Confirm cross-reference footers render and link correctly.
