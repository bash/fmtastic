//! A **fantastic** crate for **fmt**ing numbers using the appropriate unicode characters via the [`Display`](core::fmt::Display) trait. ✨ \
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
//! # Ballot Box
//! Formats a boolean as a ballot box.
//!
//! ```
//! # use fmtastic::BallotBox;
//! assert_eq!("☑ Buy bread", format!("{} Buy bread", BallotBox(true)));
//! assert_eq!("☐ Do the dishes", format!("{} Do the dishes", BallotBox(false)));
//! assert_eq!("☒ Laundry", format!("{:#} Laundry", BallotBox(true)));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

/// An abstraction over all integer types.
/// Integers can be formatted as [`Subscript`], [`Subscript`] or [`VulgarFraction`].
///
/// Use this trait if you want to abstract over integers that can be formatted
/// by one of this crate's formats:
///
/// ```
/// use fmtastic::{Subscript, Integer};
///
/// assert_eq!("x₁", x_with_index(1u8));
/// assert_eq!("x₅", x_with_index(5u64));
///
/// fn x_with_index<T: Integer>(index: T) -> String {
///     format!("x{}", Subscript(index))
/// }
/// ```
#[allow(private_bounds)]
pub trait Integer: ToIntegerImpl {}

/// Abstraction over signed integer types.
pub trait SignedInteger: Integer {}

/// Abstraction over unsigned integer types.
/// Unsigned integers can be formatted as [`Segmented`] or [`TallyMarks`].
pub trait UnsignedInteger: Integer {}

pub(crate) trait ToIntegerImpl {
    type Impl: crate::integer::IntegerImpl<Public = Self>;

    fn to_impl(&self) -> Self::Impl;
}

mod sub_superscript;
pub use sub_superscript::*;
mod fraction;
pub use fraction::*;
mod integer;
mod tally_marks;
pub use tally_marks::*;
mod seven_segment;
pub use seven_segment::*;
mod ballot_box;
pub use ballot_box::*;

mod digits;

#[doc = include_str!("../readme.md")]
#[cfg(doctest)]
pub mod readme_doctest {}
