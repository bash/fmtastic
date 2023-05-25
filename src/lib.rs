//! A **fantastic** crate for **fmt**ing numbers using the appropriate unciode characters via the [`Display`][`std::fmt::Display`] trait. ✨ \
//! Supports vulgar fractions, super- and subscript.
//!
//! Contributions are welcome for more formats.

mod sub_superscript;
pub use sub_superscript::*;
mod fraction;
pub use fraction::*;
