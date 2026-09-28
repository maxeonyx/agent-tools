# CrossCut concerns

Each file in `concerns/` is an engineering question we decided is worth being able to ask again about the agent-tools ecosystem. That ecosystem is this umbrella, the tool and library repositories pinned under `tools/` and `libraries/`, their sites and releases, and the machines and agents that actually run them. Each file says why the question matters here, how to get a current view, and what the latest dated view is. Git history holds the earlier views. `projects.md` lists what the concerns look at, including the parts that are not repositories.

These are visibility and food for thought, not obligations. Any of them can be acted on, deferred, accepted as a trade-off, designed away, or ignored. A view that reports bad news is a successful view. Nothing here is a standard, a gate or a score.

This directory is where agent-tools starts to become an application of CrossCut. The older concern machinery, `crates/standards` with its ratchet ledger, attestations in `state.json` and the process in the root `AGENTS.md`, is still in place. Here it is treated as evidence, not authority. One of the concerns (`concern-machinery-yield`) looks at that machinery directly. None of these files depends on it, and none of them feeds it.

To refresh a concern, follow its "How to look" section and replace its "Current view" section, by hand or with a coding agent. The CrossCut tool (`crosscut refresh`) can do this headless, but nothing here depends on it. Read the previous view before you replace it. Git keeps the old one.

## Doctrine (carry this forward)

CrossCut exists to widen what an engineering agent notices about software: the dimensions that are easy to miss on the direct path from idea to working product.

Its product is visibility and food for thought, never obligation. Every observation leaves the human free to act, defer, accept the trade-off, redesign the concern away, hand it to someone else, or ignore it.

Applicability and importance are contextual. A common concern is not a universal one. Unknown is not bad. Not applicable is not good. Deliberately accepted is not forgotten.

Expand the concern landscape aggressively and concretely. Lead by example rather than saying "think broadly".

Get each current view by the lightest strong mechanism:
1. First ask whether the concern can disappear through better design, or become structural so the bad state cannot happen.
2. Then prefer a mature existing tool.
3. Build custom deterministic machinery only when it is high-signal and low-maintenance.
4. Use agent judgment where the property genuinely needs judgment, and treat that as a proper mechanism, not a stopgap.

Concerns and their mechanisms evolve, and may be deleted. Minimise total complexity, including CrossCut's own. CrossCut applies itself to itself. Agent ergonomics are product ergonomics.

CrossCut is not a standard, a gate, a scorecard or a checklist. A refresh that finds bad news has succeeded.

**Anything that carries this doctrine forward must carry both the doctrine and this requirement to carry both forward again.** That includes a prompt, a README, a delegated agent's instructions, a generated file, and a rewrite of this text. Dropping either one is a defect.

## Keeping the framing

Anyone or anything that changes, extends or regenerates these files must keep this framing, and must pass on the requirement to keep both the framing and this requirement.
