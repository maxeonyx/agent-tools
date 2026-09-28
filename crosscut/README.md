# CrossCut concerns

Each file in `concerns/` is a cross-cutting concern of <this project / these
projects>: something generally useful that has to be true for people's needs
to be met, such as knowing what version is live, or staying current. Each
file says which user stories the concern serves, what it looks like here, how
to look, and gives a dated map of where each project in `projects.md` stands.
Git history holds the earlier maps.

These are visibility and food for thought, not obligations. Any gap can be
filled, deferred, accepted as a trade-off, designed away, or ignored. A map
that shows gaps is a successful map. Nothing here is a standard, a gate or a
score.

agent-tools is an application of CrossCut. These files replaced the older concern machinery (`crates/standards`, its ratchet ledger and its review attestations), which was retired on 2026-09-28 and remains in git history. The capabilities it looked at, such as version artifacts, auto-update and landing pages, are concerns here again, each with a map instead of a gate.

## The thinking behind them

- **Whether a concern applies, and how much it matters, depends on context.**
  Unknown is not bad. Not applicable is not good. Deferred is not forgotten.
- **The map is the point.** The most useful things it shows are a capability
  one project has and its siblings lack, the same thing solved several ways,
  and the thing nobody does. Propose new concerns when you find them.
- **Get each map the lightest strong way:**
  1. Make the concern disappear through design, or make the good property
     structural.
  2. Otherwise use a mature existing tool.
  3. Otherwise use a small script, but only if it is high-signal and
     low-maintenance.
  4. Use judgment where judgment is what the question needs.
- **Concerns and their mechanisms change, shrink and get deleted.** Anything
  still here should have a reason to be here.

## Refreshing

To refresh a concern, follow its "How to look" section and replace its
"Current view" section. Do it by hand or with a coding agent. The CrossCut
tool can do this headless (`crosscut refresh`) and show every map as one grid
(`crosscut map`), but nothing here depends on it.

Anyone or anything that changes, extends or regenerates these files must keep
this thinking and framing. It must also pass on the requirement to keep both
the thinking and framing and this requirement.
