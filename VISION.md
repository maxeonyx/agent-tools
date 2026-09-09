# agent-tools Vision

`agent-tools` is the control plane for a family of small, sharp agent-facing tools.

Its job is to make the quality of the whole ecosystem visible, so the quality waterline can be raised deliberately. Each cross-cutting concern is one independent aspect — release freshness, help text, injectable IO, CI evidence — and the standards suite maps where every tool sits on every one of them. Raising the waterline means picking an aspect and lifting it properly, across the suite.

A red result says where a tool sits on one aspect. It is information, and an acceptable resting state — never a debt to be cleared by any available means. Eventually, I intend to make everything green, the proper way only. There is time.

This repo should:

- make cross-cutting improvements cheaper than one-off fixes
- turn process expectations into executable checks
- track a diversity of aspects, so the map stays honest about what it does not yet measure
- keep tool repos aligned without making them the place where development happens
- evolve toward a reusable `crosscut` system other repo families can adopt

This repo should not:

- become a dumping ground for unrelated product logic
- treat prose-only guidance as sufficient enforcement
- optimize individual tool convenience at the cost of suite-wide consistency
- buy a green with a hack
