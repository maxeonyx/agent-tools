# Could a fresh engineer change each tool safely next month, and would its tests catch a mistake?

## Why this matters here

Most change to these tools is made by agents that arrive cold. What protects the tools is whether their structure makes the next change obvious, and whether their tests would catch a wrong one quickly. That means black-box tests of real behaviour, IO that tests can control, and a fast loop for the common edit. The retired suite looked at this through file-shape proxies (a `tests/` directory exists, a 5-second second run) and four review attestations, none of them current.

This applies strongly to dotsync, which holds users' dotfiles and gets the most change, and to tdd-ratchet, whose value is being trusted. It applies moderately to trunc and tb. It does not apply to oc. Better means that for a plausible change, the place to make it is clear, the edit-to-feedback loop takes seconds, and deliberately breaking the behaviour turns a test red.

## How to look

- **Deterministic, cheap:**
  - The fast loop: `time cargo test --lib` or the tool's documented fast check. Run it twice, and record the second, warm run.
  - Where recent change has gone: `git -C tools/<t> log --since='3 months ago' --format= --name-only | sort | uniq -c | sort -rn | head`.
- **Judgment:** for one tool per refresh, pick a small realistic change from its `TODO.md` or issues. Without making it, work out:
  - Where would it go, and what would you have to understand first?
  - Which tests would tell you it works?
  - Which tests would tell you that you broke something else?

  Then deliberately break one behaviour in a scratch copy (`cp -r`, never the checkout) and see whether the suite notices, and how fast.
- Things to notice on the way: hidden global state, tests that depend on the host (tb's shared tmux server), and code that exists only for compatibility with something gone.
