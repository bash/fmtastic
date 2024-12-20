//! A **fantastic** crate for **fmt**ing numbers using the appropriate unicode characters via the [`Display`](core::fmt::Display) trait. ✨ \
//! Supports vulgar fractions, super- and subscript.
//!
//! # [Vulgar Fractions]
//! Creates beautiful unicode fractions like ¼ or ¹⁰⁄₃.
//! ```
//! # use fmtastic::VulgarFraction;
//! assert_eq!("¹⁰⁄₃", format!("{}", VulgarFraction::new(10, 3)));
//! assert_eq!("¼", format!("{}", VulgarFraction::new(1, 4)));
//! ```
//!
//! # Sub- and superscript
//! Formats integers as sub- or superscript.
//!
//! ```
//! # use fmtastic::{Subscript, Superscript};
//! assert_eq!("x₁", format!("x{}", Subscript(1)));
//! assert_eq!("n²", format!("n{}", Superscript(2)));
//! ```
//!
//! # Roman Numerals
//! Formats unsigned integers as Roman numerals.
//!
//! ```
//! # use fmtastic::Roman;
//! assert_eq!("ⅾⅽⅽⅼⅹⅹⅹⅰⅹ", format!("{:#}", Roman::new(789_u16).unwrap())); // lowercase
//! assert_eq!("ⅯⅯⅩⅩⅠⅤ", format!("{}", Roman::new(2024_u16).unwrap()));
//! assert_eq!("MMXXIV", format!("{}", Roman::new(2024_u16).unwrap().ascii())); // ascii
//! assert_eq!("ⅠⅠⅠ", format!("{}", Roman::from(3_u8))); // u8's can always be formatted as Roman numeral
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
//! # Outlined
//! Formats an unsigned integer using outlined digits
//! from the [Legacy Computing Supplement] block.
//!
//! ```
//! # use fmtastic::Outlined;
//! assert_eq!("𜳶𜳲𜳸", format!("{}", Outlined(628_u32)));
//! ```
//!
//! [Legacy Computing Supplement]: https://www.unicode.org/charts/PDF/U1CC00.pdf
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
pub trait Integer: ToIntegerImpl + Copy {}

/// Abstraction over signed integer types.
pub trait SignedInteger: Integer {}

/// Abstraction over unsigned integer types.
/// Unsigned integers can be formatted as [`Segmented`] or [`TallyMarks`].
#[allow(private_bounds)]
pub trait UnsignedInteger: Integer + ToUnsignedIntegerImpl {}

pub(crate) trait ToIntegerImpl {
    type Impl: crate::integer::IntegerImpl<Public = Self>;

    fn into_impl(self) -> Self::Impl;
}

pub(crate) trait ToUnsignedIntegerImpl: ToIntegerImpl<Impl = Self::UnsignedImpl> {
    type UnsignedImpl: integer::UnsignedIntegerImpl<Public = Self>;
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
mod roman;
pub use roman::*;
mod outlined;
pub use outlined::*;

mod digits;

#[doc = include_str!("../readme.md")]
#[cfg(doctest)]
pub mod readme_doctest {}
