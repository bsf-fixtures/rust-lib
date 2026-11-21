<!--
  What has gone wrong before, and what was learned. Seeded by setup from the
  mined corrections, then added to by the scheduled review.

  This file is for failures whose lesson could not be turned into a check. A
  correction that became a rule does not belong here: the rule is the record,
  and repeating it in prose means two things to keep in step.
-->

# History — rust-lib

This repo has no history to learn from yet. That is a finding, not an omission,
so each section below says where setup looked and what was there.

## Lessons

| What happened | What to do instead | Where it came from |
| --- | --- | --- |

**None.** Setup looked at: the full `git log` (one commit, `f6ec681`, the
factory install, merged as PR #3 on 2026-11-20); `.factory/guardrails/candidates.json`,
which is `{"schema": "candidates/1", "candidates": []}`, so no corrections have
been mined from anyone's agent sessions; `.factory/definition/context/practices.md`,
which holds a header row and nothing under it; and `README.md`, which is two
lines and records no incident. There is no code, so there is no bug that has
been fixed here and no lesson from fixing one.

The first entries will come from the scheduled review (every 3 days, once at
least 10 runs are available to grade) and from humans correcting factory PRs. A
lesson belongs here only when no check could have caught it; if a check could,
it goes in `.factory/guardrails/` instead and the rule is the record.

## Problem classes already eliminated

Kept so nobody proposes a rule that exists, and so a regression is
recognisable as one.

| Problem class | How it was eliminated | When |
| --- | --- | --- |

**None.** `.factory/guardrails/baselines.json` is `{"schema": "baselines/1", "rules": {}}`
— no linter has ever run against code here, so no rule has a baseline and no
count has ever been ratcheted down. `.factory/guardrails/coverage.json` sits at
`high_pct: 0` and `floor_pct: 0`, which is the starting state, not a floor
anything has been held to.

A consequence worth knowing before proposing a hardening rule: the first run of
gate 1 against real Rust code will establish clippy's baseline at whatever that
code's warning counts happen to be, not at zero. Lowering them afterwards is
always allowed; raising one needs a trusted human.

## Escalations

Work the factory could not finish, and why. A pattern across several of these
is a signal about the factory rather than about the work.

| Work item | Where it stopped | Cause |
| --- | --- | --- |

**None.** No work item has run: this is the factory's setup, and the only
pipeline activity in the repo so far is the install commit itself.

Two limits are already known and will produce escalations if work runs into
them, so they are recorded here in advance rather than discovered twice:

- **Nothing can be verified by rendering.** The frozen sandbox is
  `build_and_test` because a library crate has no process to start. A work item
  whose acceptance depends on seeing something on a screen cannot be verified
  here, and needs an amendment that gives the sandbox a `run` command and an
  `app_url` first.
- **Coverage is not measured.** Gate 2 finds no `coverage/lcov.info` and, with
  the floor at 0, reports neutral. A work item whose acceptance is a coverage
  number cannot be judged until an amendment adds `cargo-llvm-cov` to the
  sandbox and has the test command write that file.
