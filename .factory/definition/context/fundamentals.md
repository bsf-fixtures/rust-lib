<!--
  The conventions no check enforces. Everything here is something a script
  could not decide, which is why it is prose.

  The bar for a line in this file: it has to be a judgement. If a static rule
  could catch it, it belongs in .factory/guardrails/ instead, because a rule
  is reproducible and prose is something agents argue with.

  Each line carries where it came from, so a line nobody can justify can be
  removed later.
-->

# Fundamentals — rust-lib

## Dominant patterns

For each recurring concern, the pattern most of this codebase already uses.
Follow it rather than introducing a second way of doing the same thing.

**There is no Rust code in this repo**, so no pattern could be read off the
codebase: no module layout, no error type, no naming, no test style, no logging.
Every row below is a constraint the factory's own configuration already imposes
on Rust work here, which is the nearest thing to a settled pattern that exists
today. Each one names the file it came from. When the first crate lands, the
scheduled review should replace these with patterns actually observed in `src/`.

| Concern | The pattern here | Source |
| --- | --- | --- |
| Linting | clippy, invoked as `cargo clippy --quiet --message-format=json -- -W clippy::all`. Gate 1 runs it itself, but only once a `Cargo.toml` sits at the repo root, and it ratchets each rule's count per file rather than demanding zero. So introducing a new clippy warning in a file that had none is a gate failure; a pre-existing one is not. | The static gate's linter discovery in `.factory/bin/runner.tgz`. |
| Where tests live | `tests/**`, plus `**/*.test.*` and `**/*.spec.*`, are already what policy classifies as test paths. Cargo's own layout puts integration tests in `tests/` and unit tests in a `#[cfg(test)] mod tests` beside the code, and the former is the one policy recognises. A unit test inside `src/` is therefore not seen as a test-path edit, and editing one does not need declaring; editing a file under `tests/` does. | `.factory/definition/policy.yaml`, `paths.tests` and `tests.declared_edits: required`. |
| Coverage reporting | Gate 2 reads `coverage/lcov.info` (or `coverage/coverage-summary.json`, or `coverage.xml`). Nothing here produces any of them and the floor is 0, so coverage reports neutral. A change that wants coverage to count has to make the test command write `coverage/lcov.info`, which needs `cargo-llvm-cov` added to the sandbox by an amendment. | The coverage gate's report discovery in `.factory/bin/runner.tgz`; `.factory/guardrails/coverage.json`. |
| Toolchain | Stable Rust, 1.100.0 as installed on the runner, host target `x86_64-unknown-linux-gnu`, default features. There is no `rust-toolchain.toml` pinning anything, so the toolchain is whatever the runner image ships. Code that needs a newer feature than the runner has will fail in the sandbox rather than at review. | Measured in the sandbox episode; absence of `rust-toolchain.toml` confirmed by `git ls-files`. |
| Dependencies | None are declared, so there is no established preference between crates for any concern — serialization, error handling, async, logging. The first choice in each of these is an open decision, not a convention to follow, and the spec role should treat it as one. | `git ls-files`: no `Cargo.toml`, no `Cargo.lock`. |

## Product intent

What this product is trying to be, where that would change a decision an
agent has to make.

The only statement of intent anywhere in the repo is `README.md`, in full:

> # rust-lib
> Build Software Factory: Fixture of a Rust Lib to install factory on

Read literally, that is two claims, and both change decisions:

- **It is a library, not an application.** Not a service, not a CLI. This is why
  the frozen sandbox has no `run` command, no `app_url` and a ceiling of
  `build_and_test`: there is nothing to start and nothing to screenshot. A work
  item that would add a binary, a server or a UI is changing what this repo is,
  and the verifier would not be able to exercise it until an amendment gave the
  sandbox a way to run it. Say so rather than adding one quietly.
- **It is a fixture.** It exists to have the factory installed on it and run
  against it. Nothing here serves real users or holds real data. That lowers the
  cost of a wrong call about the library's design and raises the value of the
  work being *legible* — a change whose effect the gates can actually measure is
  worth more here than a clever one they cannot.

**Nothing states what the library should do.** There is no issue, no spec, no
design note and no code implying a domain. Any agent that believes it knows the
subject of this library is guessing. The subject arrives with the first work
item, and the scheduled review should rewrite this section once it has.

## Taste

Things a reviewer would ask for that no rule states.

**Nothing yet, and nothing invented.** There is no review history to read
taste from: one commit (`f6ec681`, the factory install, merged as PR #3), no
code and so no code review, and `.factory/guardrails/candidates.json` is empty,
so no corrections have been mined from anyone's sessions either. Rust has plenty of widely-held taste, but
none of it has been stated *by the people who own this repo*, and writing it
here would dress a general opinion up as their judgement.

This section fills from three places, all of which already work:

- `bsf practice add TEXT`, or a `/practice` comment on any issue or PR, which
  lands in `practices.md` and becomes a rule where one can enforce it.
- The scheduled review, which runs every 3 days once there are at least 10 runs
  to grade, and routes what it finds.
- A human correcting a factory PR — the thing most worth capturing, because it
  is taste the factory got wrong once and would otherwise get wrong again.

## Deliberate exceptions

Places that look like they break a rule and should stay as they are, with the
reason. Without this section, cleanup keeps rediscovering them.

| Place | Looks like | Why it stays |
| --- | --- | --- |
| `.factory/bin/runner.tgz` | A binary blob committed to the repo, and a `.tgz` that no linter or formatter can see into. | Deliberate, and documented in `.factory/bin/bsf` itself: the runner is packed so that no linter, formatter or type checker in this repo reads it. It is also the program every gate runs, which is why `policy.yaml` protects `.factory/bin/**` — a PR must not be able to grade itself with a modified copy. Never unpack it, lint it, or "fix" anything inside it. |
| The frozen sandbox's `install`, `build` and `test` commands | Each is wrapped in `if [ -f Cargo.toml ]`, which reads like a command that has been defanged so a check passes. | The guard exists because there is no crate in this repo yet; bare `cargo build` exits 101 with "could not find Cargo.toml", which would have frozen the sandbox at `static` and told every role never to compile or test the Rust code the factory is here to write. The guard skips only while no `Cargo.toml` exists. From the first one onwards each command runs unguarded and a failure is a real failure. Remove the guards once a crate exists — but not before, and not by replacing them with `|| true`, which would hide real failures forever. |
| No `rust-toolchain.toml` | A Rust repo that does not pin its toolchain. | Nothing to pin against yet: there is no code with an MSRV and no dependency with a minimum. Adding one is a reasonable early work item, but it is a decision to make on purpose rather than a gap to close reflexively, and it would need the sandbox checked against whatever version it pins. |
