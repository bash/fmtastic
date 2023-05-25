//! A **fantastic** crate for **fmt**ing numbers using the appropriate unciode characters via the [`Display`](std::fmt::Display) trait. ✨ \
//! Supports vulgar fractions, super- and subscript.
//!
//! # [Vulgar Fractions]
//! Creates beautiful unicode fractions like ¼ or ¹⁰⁄₃.
//! ```rust
//! # use fmtastic::VulgarFraction;
//! assert_eq!("¹⁰⁄₃", format!("{}", VulgarFraction::new(10, 3)));
//! assert_eq!("¼", format!("{}", VulgarFraction::new(1, 4)));
//! ```
//!
//! # Sub- and superscript
//! Formats integers as sub- or superscript.
//!
//! ```rust
//! # use fmtastic::{Subscript, Superscript};
//! assert_eq!("x₁", format!("x{}", Subscript(1)));
//! assert_eq!("n²", format!("n{}", Superscript(2)));
//! ```
//!
//! [Vulgar Fractions]: https://en.wikipedia.org/wiki/Fraction_(mathematics)#Simple,_common,_or_vulgar_fractions

mod sub_superscript;
pub use sub_superscript::*;
mod fraction;
pub use fraction::*;

#[doc = include_str!("../readme.md")]
mod doctest_readme {}
