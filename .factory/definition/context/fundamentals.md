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

| Concern | The pattern here | Source |
| --- | --- | --- |
| Representing money | `Cents(i64)`, a newtype over integer cents, signed so a credit note is negative. Never `f32` or `f64`, and never a bare integer passed around as "an amount". | `src/money.rs` module doc: "Never a float. A cent is the smallest unit anyone is billed in, and `0.1 + 0.2` is the reason this type exists." README. |
| Input that can be wrong | Return `Option`, never panic. `Cents::parse` takes `&str` and gives `Option<Cents>` because "the input comes from a file or a form, so a bad value is an ordinary event, not a bug in the caller". | `Cents::parse` doc comment in `src/money.rs`; test `refuses_anything_that_is_not_money`. |
| Arithmetic that could overflow on untrusted input | Checked: `checked_mul`, `checked_add`, and `?` on the `Option` they return. | `Cents::parse` in `src/money.rs`. |
| Percentages | Basis points as `i64` — 2000 is 20% — applied through `Cents::basis_points`, which rounds half **away from zero**, not toward it and not with Rust's default truncation. | `Cents::basis_points` doc comment; `Invoice.tax_bps` field doc; test `rounds_half_away_from_zero`. |
| Where tax is taken | On the net total of the invoice, once. Never per line and then summed. | Comment inside `Invoice::total` in `src/invoice.rs`; test `taxes_the_total_rather_than_each_line`. |
| Where tests live | A `#[cfg(test)] mod tests` block at the bottom of the file it tests, with `use super::...`. There is no `tests/` directory. | Both `src/money.rs` and `src/invoice.rs`. |
| Test names | A sentence that states the rule: `rounds_half_away_from_zero`, `taxes_the_total_rather_than_each_line`, `refuses_anything_that_is_not_money`. Not `test_parse_1`. | All 7 test names in `src/money.rs` and `src/invoice.rs`. |
| Comments | `//!` module docs and `///` item docs explain **why** the rule exists, in money terms, usually naming what goes wrong without it. They do not restate the code. | Every doc comment in `src/lib.rs`, `src/money.rs`, `src/invoice.rs`. |
| Dependencies | None. `[dependencies]` and `[dev-dependencies]` are empty and `Cargo.lock` contains only this crate. `std` is the whole toolkit. | `Cargo.toml`, `Cargo.lock`. |
| Public surface | Types callers use are re-exported from `src/lib.rs`, so the module path is an implementation detail: `invoice_core::Cents`, not `invoice_core::money::Cents`. | `pub use` lines in `src/lib.rs`. |
| Deriving | Small value types derive the full set they can — `Cents` is `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord`; `Total` is `Debug, Clone, Copy, PartialEq, Eq`. Types holding a `String` or `Vec` derive `Debug, Clone` only. | `src/money.rs`, `src/invoice.rs`. |

## Product intent

This is invoice arithmetic, and the product is the arithmetic being right.
Where a decision trades elegance against a cent, the cent wins: the whole
point of `basis_points` rounding half away from zero is that always rounding
down "loses a cent on roughly half of all invoices, and the loss is
systematic rather than averaging out". A change that makes the code tidier
and moves a rounding boundary is not a tidying change.

The number a customer checks is the **total**, which is why tax is taken on
the net total rather than line by line. When a new calculation could be done
either per line or on an aggregate, do it on the aggregate the customer sees,
and say in a comment which number that is.

The second thing this repo is for, stated in its README, is being a fixture
for the factory: the case where verification has nothing to open in a browser
and nothing to curl, so the verifier has to call the code. That shapes what
"done" looks like here. A change is demonstrated by a test, or by a scratch
example that prints what the library returns (see the runbook's Verify
section) — not by a screenshot, because there is no screen.

It also means the surface should stay small enough to call directly. A change
that makes the crate awkward to exercise from a six-line `main()` has cost
something real.

## Taste

Things a reviewer would ask for that no rule states.

- **A test that names the rule it protects.** Each of the three load-bearing
  tests carries a comment saying what breaks without it — what 7.5 cents of
  tax costs across half of all invoices, why three one-cent lines at 50% come
  to 2 and not 3. A new test for a rule that matters should be able to say
  the same thing.
- **Explain the money, not the Rust.** The comments worth having here are the
  ones about invoices. A comment explaining what `checked_mul` does is noise;
  a comment explaining why an amount from a form must not panic is the point.
- **Reach for `std` before reaching for a crate.** Adding the first
  dependency to this crate is a decision, not a convenience. It changes the
  build from "needs no network" to "needs the registry", and the empty
  `[dependencies]` table is load-bearing for that reason.
- **Keep parsing strict.** `Cents::parse` demands exactly two fractional
  digits and rejects `"1.5"`. That is deliberate strictness about a money
  string, not an oversight to be relaxed into leniency; loosening it is a
  product decision, not a bug fix.
- **Prefer a new module over a longer one.** `src/` is flat with three small
  files. A new concern gets `src/<name>.rs` and a `pub mod` line rather than
  another two hundred lines in `money.rs`.
- **Say what a function returns in money, not in types.** `Total` has `net`,
  `tax` and `gross` as three named fields rather than a tuple, because the
  caller has to know which is which.

## Deliberate exceptions

Places that look like they break a rule and should stay as they are, with the
reason. Without this section, cleanup keeps rediscovering them.

| Place | Why it stays |
| --- | --- |
| `clippy::unwrap_used` is `warn` in `Cargo.toml`, not `deny`, even though CI runs clippy with `-D warnings`. | The README says so explicitly: it is set to `warn` "so the rule pack's baseline has something real to ratchet against". Raising it to `deny` removes the thing the static gate measures. Leave the level alone; the gate is what enforces it. |
| `Cents` has a public tuple field (`pub struct Cents(pub i64)`) rather than an accessor. | Every test and every arithmetic site constructs and reads cents directly — `Cents(1234)`, `self.0 * bps`. Hiding the field behind `new()` and `get()` would add ceremony to a type whose whole job is to be a number, and would make the tests harder to read than the rule they state. |
| `Cents::basis_points` uses plain `*` and `/` while `Cents::parse` uses `checked_mul`/`checked_add`. | The two have different inputs. `parse` takes a string from a file or a form, so an absurd value is an ordinary event it must survive. `basis_points` takes amounts the library itself has already produced. This is an observation about the current design, not a verdict: if a change makes `basis_points` reachable from untrusted input, the asymmetry stops being justified. |
| `Line::subtotal` multiplies by `i64::from(self.quantity)` from a `u32` quantity, rather than taking an `i64`. | A quantity is not signed — there is no negative quantity of a line — while an amount is, because a credit note is negative. The conversion at the boundary is the cheap way to keep both facts in the types. |
| No `tests/` directory, no integration tests. | The crate's public surface is three types and five methods, all of which the in-module unit tests reach. An integration test here would re-test the same code through a longer path. |
| `license = "MIT"` with `publish = false`. | It is a fixture, not a crate anyone installs. The licence is stated for the people reading the repo; publishing is off so nothing tries to release it. |
| Doc comments in all three files are long relative to the code. | They carry the rules — half away from zero, tax on the total, never a float — that the fundamentals above restate. They are the reason a change to this crate can be reviewed at all. Shortening them is a loss, not a cleanup. |
