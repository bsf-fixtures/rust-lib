<!--
  How to build, run, test and verify this app. Written by setup's phase 2,
  with every command tested rather than copied from a README.

  This is the most load-bearing context file: a verifier agent that cannot
  start the app cannot verify anything, and then the factory produces
  confident unverified PRs.
-->

# Runbook — rust-lib

## Build

```bash
cargo build --all-targets
```

Finishes in well under a second from cold. There is nothing to install first:
`[dependencies]` and `[dev-dependencies]` in `Cargo.toml` are both empty, so
the build needs no network. The sandbox still runs an install step —
`rustup component add clippy && cargo fetch --locked` — because the static
gate runs clippy, and because `cargo fetch --locked` is the step that will
fail loudly the day `Cargo.lock` stops matching `Cargo.toml`.

`--all-targets` rather than a bare `cargo build`: it compiles the test
targets and any example too, so a broken test file is a build failure rather
than a surprise at test time. This matches `.github/workflows/ci.yml`.

## Run

```bash
# There is no run command. invoice-core is a library crate.
```

The app is then at **nowhere — there is no URL**. `Cargo.toml` declares no
`[[bin]]`, no file in `src/` has a `main()`, and nothing in the crate opens a
socket or listens on a port. This is deliberate and is the repo's stated
reason for existing. Verification here means calling the code; see **Verify**
below. The sandbox manifest records `run: null`, `app_url: null`, and tier
`build_and_test`.

## Test

```bash
cargo test --all-targets
```

7 tests, all passing, finishing in milliseconds: 4 in `money::tests` and 3 in
`invoice::tests`. They live in `#[cfg(test)] mod tests` blocks at the bottom
of the module each one tests; there is no `tests/` directory.

Three of them state rules a change could quietly break, and are the ones to
read before changing anything in `src/money.rs` or `src/invoice.rs`:

- `refuses_anything_that_is_not_money` — `Cents::parse` returns `None` for
  `""`, `"abc"`, `"1.5"`, `"1.234"`, `"1,00"` and `"."`. It must not panic.
- `rounds_half_away_from_zero` — `Cents(100).basis_points(750)` is `Cents(8)`
  and `Cents(-100).basis_points(750)` is `Cents(-8)`. Not 7, in either
  direction.
- `taxes_the_total_rather_than_each_line` — three lines of one cent at 5000
  bps give 2 cents of tax, not 3.

`--all-targets` does **not** run doctests. None exist today — the doc
comments are prose with no ``` examples — but a doc example added later needs
`cargo test --doc` and will not be covered by the command above.

Lint separately, which is what the static gate does:

```bash
cargo clippy --all-targets -- -D warnings
```

Currently clean. Note `clippy::unwrap_used` is set to `warn` in `Cargo.toml`,
so an `unwrap()` added to `src/` is a warning the baseline comparison will
catch rather than a hard build failure.

## Verify

How to see for yourself that the app works, as a developer would. There is
nothing to open, so the only honest verification is to call the library and
read what comes back.

1. **Run the tests and read the names.** `cargo test --all-targets` prints
   all 7 test names. The names are sentences; a failing one tells you which
   rule broke without opening the file.

2. **Call the code from a scratch example.** This is the closest thing to
   "opening the app". From the repo root:

   ```bash
   mkdir -p examples && cat > examples/scratch_check.rs <<'EOF'
   use invoice_core::{Cents, Invoice, Line};

   fn main() {
       let invoice = Invoice {
           number: "INV-9".into(),
           lines: vec![
               Line { description: "design".into(), quantity: 3,
                      unit_price: Cents::parse("19.99").expect("amount") },
               Line { description: "hosting".into(), quantity: 1,
                      unit_price: Cents::parse("5.00").expect("amount") },
           ],
           tax_bps: 2000,
       };
       let t = invoice.total();
       println!("net   {}", t.net.to_string_amount());
       println!("tax   {}", t.tax.to_string_amount());
       println!("gross {}", t.gross.to_string_amount());
       println!("bad input -> {:?}", Cents::parse("1.5"));
   }
   EOF
   cargo run --example scratch_check
   ```

   Run here during setup, it prints exactly:

   ```
   net   64.97
   tax   12.99
   gross 77.96
   bad input -> None
   ```

   Check the arithmetic by hand: `3 × 19.99 + 5.00 = 64.97`; 20% of 6497
   cents is 1299.4, which rounds to 1299; `64.97 + 12.99 = 77.96`.

3. **Delete the scratch example when you are done**: `rm -rf examples`.
   Because build and test both use `--all-targets`, a leftover example is
   compiled by every later build, and a verifier's throwaway file then
   becomes something the gates have an opinion about. It is also not covered
   by `.gitignore`, which ignores only `/target` and `.playwright-cli/`.

4. **Note what the re-exports allow.** `use invoice_core::{Cents, Invoice,
   Line};` is enough — `src/lib.rs` re-exports them, so you do not need to
   name the `money` or `invoice` modules. If a new type is not reachable that
   way, it was not added to the re-export list.

## Seed data

```bash
# There is none, and none is needed.
```

Nothing in this crate reads a file, a database or any other store. The data
an `Invoice` operates on is constructed by the caller in memory, as in the
scratch example above. There is no fixture directory and no migration.

## Commands that change persistent state

These are kept out of the build path on purpose. Each one writes to something
that outlives the process, so running it against the wrong target is the kind
of mistake that cannot be undone by reverting a commit.

| Command | Writes to | Only when |
| --- | --- | --- |
| — | — | This repo has none. There is no migration, no deploy script, no seeding step and no release step. `cargo publish` would be the only candidate and `Cargo.toml` sets `publish = false`, so it refuses to run. |

## Services this app needs

| Service | How it starts | Reached at |
| --- | --- | --- |
| — | — | None. The crate has no dependencies at all, so there is no database, cache, queue, object store or mail catcher to start. The sandbox manifest's `services` list is empty because the app genuinely needs nothing, not because something could not be reached. |

## The sandbox's environment

Built from this list, and from nothing else. It does not inherit a
developer's environment and it does not read any `.env` file, because a
documented build command that runs a migration would otherwise migrate
whatever a developer's credentials happen to point at.

| Variable | Value in the sandbox |
| --- | --- |
| `RUST_BACKTRACE` | `1` — the test harness's own setting, so a panic introduced by a change names its frame instead of asking you to re-run. Nothing in the library reads it. |

That is the whole list. Grepping `src/` for `env::var`, `env!`,
`option_env!` and `std::env` returns nothing: the crate reads no environment
variable, which is why there is no secret, no `test_mode` entry and no
`inert` entry in the manifest. The sandbox may reach
`static.rust-lang.org` and the crates.io hosts for the toolchain and for
dependencies a change adds; nothing else.
