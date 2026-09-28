# agent-tools Vision

`agent-tools` is the control plane for a family of small, sharp agent-facing tools.

Its job is to make the quality of the whole ecosystem visible, so the quality waterline can be raised deliberately. Cross-cutting concerns — what people actually end up installing, whether the agent guidance is true, what runs with release authority, and whatever nobody has thought of yet — live as [CrossCut](tools/crosscut) concern files in `crosscut/concerns/`: questions worth asking again, each with a dated view of where the suite sits. Raising the waterline means picking one and lifting it properly, across the suite.

A view that finds a weakness is information, and an acceptable resting state — never a debt to be cleared by any available means. Eventually, I intend to lift everything that matters, the proper way only. There is time.

This repo should:

- make cross-cutting improvements cheaper than one-off fixes
- keep a diversity of concerns visible, each through the lightest mechanism that gives a real view
- keep looking for the concerns it does not yet represent
- keep tool repos aligned without making them the place where development happens
- be the first real application of CrossCut, which other repo families can adopt

This repo should not:

- become a dumping ground for unrelated product logic
- turn concerns into gates, scores or compliance machinery; it tried that, and it became machinery for the machinery
- optimize individual tool convenience at the cost of suite-wide consistency
- buy a green with a hack
