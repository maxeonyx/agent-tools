# A front door

Within a minute, someone can tell what each project is, who it is for, and why it exists.

## User stories

- **A visitor to a tool's site:** what is this, should I care, how do I start?
- **Max, choosing which tool to point an agent at:** the umbrella site and README should say what each tool is for.

## What it looks like here

- Each tool: a site at `<tool>.maxeonyx.com` and a README pointing to it.
- The umbrella: tools.maxeonyx.com and its README list every maintained tool and link to it.
- An archived tool's front door says it is archived and why.

## How to look

- `curl -s -o /dev/null -w '%{http_code}' https://<site>/` for each project, and read each landing page.
- Do the umbrella's README and site list every project in `crosscut/projects.md`?
- Judgment: read each front door cold. Does it answer what, for whom, why, and how to start, in a minute?
