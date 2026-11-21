<!--
  What has gone wrong before, and what was learned. Seeded by setup from the
  mined corrections, then added to by the scheduled review.

  This file is for failures whose lesson could not be turned into a check. A
  correction that became a rule does not belong here: the rule is the record,
  and repeating it in prose means two things to keep in step.
-->

# History — rust-lib

Setup found no history to seed this file from, and has not invented any. What
it looked at, so the next person does not look again:

- `git log` holds a single commit on `main`,
  `4966764 Upgrade the factory to build-software-factory ecee54e (#7)`. There
  is no record of a change to `src/` being made, reverted or corrected.
- `.factory/guardrails/candidates.json` is `{"schema": "candidates/1",
  "candidates": []}` — no mined corrections, so there are no past
  agent-session corrections to turn into lessons.
- `.factory/guardrails/baselines.json` has an empty `rules` map, so no rule
  has yet recorded a count to ratchet from.
- There are no open or closed work items, runs or escalations to read: this
  is the factory's first setup on this repo.

The rules this repo cares about are written down in the code itself — never a
float for money, round half away from zero, tax the total rather than each
line — and each has a test enforcing it. Those are checks, not history, so
they belong in `src/` and in `fundamentals.md`, not here.

The first real entries will come from the scheduled review, once runs have
been scored.

## Lessons

| What happened | What to do instead | Where it came from |
| --- | --- | --- |
| — | — | Nothing yet. No change to this repo has been made and corrected. |

## Problem classes already eliminated

Kept so nobody proposes a rule that exists, and so a regression is
recognisable as one.

| Problem class | How it was eliminated | When |
| --- | --- | --- |
| — | — | Nothing eliminated by the factory yet. The three correctness rules the code already enforces were there before the factory arrived and are covered by tests, not by a factory rule — see `fundamentals.md` and the tests `rounds_half_away_from_zero`, `taxes_the_total_rather_than_each_line` and `refuses_anything_that_is_not_money`. |

## Escalations

Work the factory could not finish, and why. A pattern across several of these
is a signal about the factory rather than about the work.

| Work item | Where it stopped | Cause |
| --- | --- | --- |
| — | — | None. No work item has been run here. |
