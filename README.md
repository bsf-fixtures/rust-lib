# rust-lib

Build Software Factory fixture: a Rust library with no binary and no server.

It exists for the case the other fixtures do not cover — **runtime
verification where there is nothing to open and nothing to curl**. The
verifier has to call the code and check what comes back, which is the shape
of verification for a library, a CLI or anything else headless.

The subject is invoice arithmetic in integer cents, so the rules worth
enforcing are about correctness rather than style: never a float for money,
round half away from zero, tax the total rather than each line.

## Build and test

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

## What the factory should find here

- `clippy::unwrap_used` is a warning in `Cargo.toml`, so the rule pack's
  baseline has something real to ratchet against.
- The tests state three things a change could quietly break: that a bad
  amount returns `None` rather than panicking, that rounding is half away
  from zero, and that tax is taken on the total.
