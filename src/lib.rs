//! A **fantastic** crate for **fmt**ing numbers using the appropriate unicode characters via the [`Display`](std::fmt::Display) trait. ✨ \
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
//!
//! # Seven-Segment Digits
//! Formats an unsigned integer using seven-segment digits
//! from the [Legacy Computing] block.
//! ```
//! # use fmtastic::Segmented;
//! assert_eq!("🯶🯲🯸", format!("{}", Segmented(628_u32)));
//! ```
//!
//! [Legacy Computing]: https://www.unicode.org/charts/PDF/U1FB00.pdf
//!
//! # Tally Marks
//! Formats an unsigned integer as tally marks.
//!
//! ```
//! # use fmtastic::TallyMarks;
//! assert_eq!("𝍷𝍷𝍷", TallyMarks(3_u32).to_string());
//! assert_eq!("𝍸𝍸𝍷𝍷", TallyMarks(12_u32).to_string());
//! ```
//!

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod sub_superscript;
pub use sub_superscript::*;
mod fraction;
pub use fraction::*;
mod integer;
pub use integer::*;
mod tally_marks;
pub use tally_marks::*;
mod seven_segment;
pub use seven_segment::*;

mod utils;

#[doc = include_str!("../readme.md")]
#[cfg(doctest)]
pub mod readme_doctest {}
