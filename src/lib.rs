// Crate-wide Clippy lints
#![deny(missing_docs)]

pub mod algo;
pub mod error;
pub mod math;
#[allow(unused_imports, unused_variables, dead_code)] // TEMP lint ignores
pub mod partition;
