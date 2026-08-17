---
title: Everything here is legal
note: frontmatter is not prose and is never judged
---

# Unwrapped markdown

This paragraph is one long line, which is the whole point of the convention, and it keeps going well past any column a wrapping tool would have chosen so that nothing about it looks filled.

Short paragraph.

One thought per line is legal even without blank lines between them.
this is a second thought, deliberately on its own line, and much longer than the line above it so no consistent fill column can be inferred
short again

- A list item that says everything it needs to say on exactly one line.
- Another item.
- Third item, which is considerably longer than its siblings but still a single line.

> A blockquote whose lines are each a separate short item
> user opens file
> user edits file
> user searches

https://github.com/example/one/blob/main/packages/thing/src/registry/anthropic.ts
https://github.com/example/two/blob/main/packages/thing/src/registry/oauth/anthropic.ts
https://github.com/example/three/blob/main/packages/thing/src/providers/anthropic-wire.ts

| Command | Purpose |
| --- | --- |
| `a` | does a thing |
| `b` | does another thing |

Invariants numbered by hand, where a sub-item keeps the author's numbering:

**Width consistency:**

5. Every row inside a panel renders to exactly the panel content width, padded if shorter.
6. This applies to all row types: data rows, group headers, separators, totals.
   6a. Footer rows are pinned to the bottom of their panel, so slack appears above them.

Code must never be read as wrapped prose, however it is written:

```bash
echo "this line is deliberately close to a fill column so it looks wrapped"
echo "and this one continues it, which outside a fence would be a violation"
```

    an indented code block whose lines also look like they were wrapped
    at a consistent column, but which must never be flagged as prose

Text with an explicit hard break is legal, because the break was asked for:\
this line follows a backslash break and is not a wrapping artifact.
