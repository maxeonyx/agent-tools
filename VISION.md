# agent-tools Vision

`agent-tools` is the control plane for a family of small, sharp agent-facing tools.

Its job is not just to hold code. Its job is to make shared standards, shared release discipline, and shared product improvements the default path across the whole suite.

This repo should:

- make cross-cutting improvements cheaper than one-off fixes
- turn process expectations into executable checks
- keep tool repos aligned without making them the place where development happens
- evolve toward a reusable `crosscut` system other repo families can adopt

This repo should not:

- become a dumping ground for unrelated product logic
- treat prose-only guidance as sufficient enforcement
- optimize individual tool convenience at the cost of suite-wide consistency
