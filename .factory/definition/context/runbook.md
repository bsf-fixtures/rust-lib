<!--
  How to build, run, test and verify this app. Written by setup's phase 2,
  with every command tested rather than copied from a README.

  This is the most load-bearing context file: a verifier agent that cannot
  start the app cannot verify anything, and then the factory produces
  confident unverified PRs.
-->

# Runbook — rust-lib

**Read this first.** There is no application in this repo yet. No `Cargo.toml`,
no `src/`, no `tests/`. The commands below are the real cargo commands and they
are what the frozen sandbox runs, but each is guarded on the `Cargo.toml` that
does not exist yet, so today each one prints a line and exits 0. The first
commit that adds a `Cargo.toml` at the repo root makes all of them bite at once,
and from that point a failure is a real failure. Remove the guards then — and
never replace them with `|| true`, which would hide failures permanently.

Nothing below was copied from a README. The README is two lines and documents no
command. Every command here was run: the guarded form in this workspace, and the
unguarded form against a throwaway crate with a real crates.io dependency, a
unit test and a doc-test.

## Build

```bash
if [ -f Cargo.toml ]; then cargo build --workspace --all-targets; else echo 'no Cargo.toml at the repo root yet - nothing to build'; fi
```

Installing first is a separate step, and the sandbox runs it before the build:

```bash
rustup component add clippy rustfmt >/dev/null 2>&1 || echo 'clippy/rustfmt: keeping the preinstalled toolchain'; if [ -f Cargo.toml ]; then cargo fetch; else echo 'no Cargo.toml at the repo root yet - nothing to fetch'; fi
```

**Notes.**

- `--workspace` is correct for a single crate as well as for a workspace, so
  neither command needs changing if the repo later grows members.
- `--all-targets` builds the lib plus every bin, example, test and bench target
  the crate declares. Without it, code that only tests compile is not built, and
  a break there surfaces at test time instead of build time.
- `--locked` is deliberately **not** used: `Cargo.lock` does not exist yet and
  `--locked` would fail on the very first build. Once a lockfile is committed,
  adding `--locked` to both `cargo fetch` and `cargo build` is a worthwhile
  tightening — it turns a silently-updated dependency into a build failure.
- The toolchain is whatever the runner image ships; measured at setup it was
  cargo and rustc 1.100.0, clippy 0.1.100, rustfmt 1.11.0, host target
  `x86_64-unknown-linux-gnu`. Nothing pins it — there is no
  `rust-toolchain.toml`.
- The `rustup component add` is idempotent and non-fatal. Both components were
  already installed when this was measured; the fallback exists so a runner
  image without them, or without network to `static.rust-lang.org`, does not
  fail the install step and drop the sandbox to `static`.

## Run

```bash
# There is no run command. This is a library crate: no binary, no server.
```

**The app is not at any URL.** `run` and `app_url` are both null in the frozen
sandbox, and the sandbox's tier is `build_and_test` because of it. There is
nothing to start, nothing to open in a browser and nothing to screenshot.

If the repo ever gains a bin target or an example server, that is a change to
what this repo is (see `fundamentals.md`, Product intent) and the sandbox has to
be amended before the verifier can exercise it — a `run` command, an `app_url`,
and then the tier can move to `renders`.

## Test

```bash
if [ -f Cargo.toml ]; then cargo test --workspace; else echo 'no Cargo.toml at the repo root yet - no tests to run'; fi
```

**Notes.**

- Plain `cargo test` — not `--all-targets` — on purpose. `--all-targets`
  silently **skips doc-tests**, and for a library the `///` examples are part of
  the public surface and the most common thing to let rot. Verified on a
  throwaway crate: `cargo test --workspace` ran one unit test and one doc-test;
  both are reported in separate sections of the output.
- Unit tests live in a `#[cfg(test)] mod tests` beside the code in `src/`;
  integration tests live in `tests/`. Only the latter is what `policy.yaml`
  classifies as a test path, so editing a file under `tests/` has to be declared
  in the spec's `tests_changed` and editing a `#[cfg(test)]` block in `src/`
  does not.
- **Coverage is not measured.** Gate 2 looks for `coverage/lcov.info` and this
  command does not write one. With `floor_pct` at 0 in
  `.factory/guardrails/coverage.json` the gate reports neutral rather than
  failing, so nothing is blocked — but no coverage number here means anything.
  Producing one needs `cargo-llvm-cov`, which is not on the toolchain and takes
  minutes to compile from source, so it was left out of the sandbox. Adding it
  is an amendment: put the install in the sandbox's `install` command and make
  `test` run `cargo llvm-cov --workspace --lcov --output-path coverage/lcov.info`.
- `RUST_BACKTRACE=1` is set in the sandbox, so a panicking test prints a
  backtrace rather than only the assertion line.

## Verify

How to see for yourself that the app works, as a developer would:

There is no app to look at, so verification here is entirely the compiler and
the test binaries. For a library that is a real answer, not a lesser one — but
be clear about what it does and does not establish.

1. **Build it.** Run the build command above. A clean build means every target
   compiles, including tests and examples. For Rust this carries more than it
   would elsewhere: type and borrow errors are caught here, not at runtime.
2. **Run the tests and read the output, not the exit code.** `cargo test`
   prints a result line per section — unit tests, each integration test binary,
   then `Doc-tests`. Check that the section you expected to grow actually ran
   tests. A test module that was never declared with `mod tests;`, or an
   integration file that was never added under `tests/`, compiles and reports
   `0 passed` rather than failing. Exit code 0 over zero tests is the most
   common false pass here.
3. **Check for `ignored`.** `cargo test` reports ignored tests in its count.
   The factory caps skips at 10 (`coverage.json`), and a `#[ignore]` added to
   get a change through is exactly what that cap is for.
4. **Run clippy the way gate 1 will**, so its verdict is not a surprise:
   `cargo clippy --quiet --message-format=json -- -W clippy::all`. The gate
   compares each rule's count per file against the baseline on the base ref, so
   what matters is whether the change *introduces* a warning in a file, not
   whether the file has any.
5. **Check formatting if it is in scope**: `cargo fmt --check`. No gate runs
   this today — there is no `rustfmt.toml` and no formatting rule in the
   guardrails — so it is a courtesy, not a requirement.
6. **Say what you could not check.** A change whose effect is only visible at
   runtime, over a network, or on a screen cannot be verified in this sandbox.
   Report that plainly rather than inferring it from a green build. The frozen
   manifest's `capabilities.not_exercised` is the list of those cases.

## Seed data

```bash
# There is no seed command, and nothing to seed.
```

No database, no cache, no object store, no fixtures directory and no migrations.
The frozen sandbox declares `services: []`, and `seed` is null. A Rust library
that acquires state in its tests should create it in a `tempfile`-style
directory inside the test and drop it at the end, rather than seeding anything
that outlives a test run.

## Commands that change persistent state

These are kept out of the build path on purpose. Each one writes to something
that outlives the process, so running it against the wrong target is the kind
of mistake that cannot be undone by reverting a commit.

**No such command exists in this repo today** — there is no release script, no
deploy workflow, no migration and no publish step. The rows below are the ones
to watch for as a Rust library grows, and the first is one the factory's own
runner already detects by pattern, which is why it is named here rather than
waiting to be discovered.

| Command | Writes to | Only when |
| --- | --- | --- |
| `cargo publish` | crates.io. A published version is **permanent**: it cannot be deleted, only yanked, and the version number can never be reused. | Never from a factory pipeline, and never from the sandbox — the sandbox has no credential that could authenticate it, by design. A human releases, deliberately, from a tagged commit. |
| `cargo login` / writing `~/.cargo/credentials.toml` | The machine's cargo credential store, outside the repo. | Never here. The sandbox environment is closed and holds no registry token. |
| `cargo yank` | crates.io's view of an already-published version, for everyone depending on it. | Never from a pipeline. A human, in response to a real problem. |
| `git push --force`, branch deletion | The remote, outside this checkout. | Never from a role. The factory opens PRs and the merge queue merges them. |

## Services this app needs

| Service | How it starts | Reached at |
| --- | --- | --- |

**None.** The frozen sandbox declares `services: []`. Nothing in this repo
connects to a database, cache, queue, object store, mail server or third-party
API, because there is no code to do so. `stubs` is empty for the same reason:
there is no client whose requests a stub could answer.

The sandbox's egress is limited to the hosts cargo and rustup actually need —
`crates.io`, `index.crates.io`, `static.crates.io`, `static.rust-lang.org` —
and nothing else. Code that reaches any other host will fail in the sandbox,
which is the intended behaviour: it is the signal to add a service or a stub
through an amendment, not to widen egress.

## The sandbox's environment

Built from this list, and from nothing else. It does not inherit a
developer's environment and it does not read any `.env` file, because a
documented build command that runs a migration would otherwise migrate
whatever a developer's credentials happen to point at.

| Variable | Value in the sandbox |
| --- | --- |
| `RUST_BACKTRACE` | `1` — a panicking test prints a backtrace, so a failing verifier run says where it broke rather than only that it broke. |
| `CARGO_TERM_COLOR` | `never` — cargo's output is captured into step logs and into the trial's tail of the last 40 lines, and ANSI colour escapes make both unreadable. |

That is the whole list. **No application variable is set, because no code reads
one.** Not a database URL, not an API key, not an origin. If the factory writes
code that reads an environment variable, it will be absent here and the code
will fail loudly — that is the design, and the failure is the signal to amend
the manifest rather than something to work around with a default.

Six variables do reach the sandbox from the runner itself and are not in the
manifest: `PATH`, `HOME`, `TMPDIR`, `LANG`, `CI` and `RUNNER_TEMP`. They are how
`cargo` is found on `PATH` and how `~/.cargo` is located; do not rely on
anything else from the ambient environment, because nothing else is there.
