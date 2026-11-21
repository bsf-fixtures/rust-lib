//! Invoice arithmetic in integer cents.
//!
//! A library rather than a binary, on purpose: it is the fixture for runtime
//! verification where there is nothing to open in a browser and nothing to
//! curl. The verifier has to call the code.

pub mod money;
pub mod invoice;

pub use invoice::{Invoice, Line, Total};
pub use money::Cents;
