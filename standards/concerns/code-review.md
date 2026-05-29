---
title: Code Review Completed
type: gate
applies_to: all
checker: "one-time: thermonuclear review loop completed and findings addressed"
---

CI passing is necessary but not sufficient. A tool can pass all automated checks and still be a mess — unclear architecture, unnecessary complexity, silent failure paths, dead code, design decisions that will cause pain later.

This gate proves someone has looked at the entire codebase with fresh eyes, challenged every decision, and fixed what was wrong. It's not ongoing maintenance — it's a one-time proof that the review loop was run.

Load the `thermonuclear-review` skill. Do the review. Fix the findings. Record the commit.

If the tool changes significantly later, the staleness detection in the standards crate will flag it for re-review.
