use crate::integer::{Integer, Sign};
use crate::{Subscript, Superscript};
use std::fmt::{self, Write};

/// A [Vulgar Fraction] that can be formatted as a unicode fraction using the [`Display`][`std::fmt::Display`] trait.
///
/// [`Display`][`std::fmt::Display`] is implemented for all common number types.
///
/// ## Formatting Flags
/// ### Alternate `#`
/// By default [single character fractions] are used when possible.
/// This can be disabled by using the alternate flag (`#`).
///
/// ### Sign: `+` and/or `-`
/// Use the `+` and/or `-` flag to move the sign to the outside of the fraction.
///
/// ## Examples
/// ```
/// # use fmtastic::VulgarFraction;
/// assert_eq!("¹⁰⁄₃", format!("{}", VulgarFraction::new(10, 3)));
/// assert_eq!("¼", format!("{}", VulgarFraction::new(1, 4)));
///
/// // Sign in front of fraction
/// assert_eq!("+¹⁰⁄₃", format!("{:+}", VulgarFraction::new(10, 3)));
/// assert_eq!("+¹⁰⁄₃", format!("{:+}", VulgarFraction::new(-10, -3)));
/// assert_eq!("-¹⁰⁄₃", format!("{:-}", VulgarFraction::new(-10, 3)));
/// assert_eq!("-¹⁰⁄₃", format!("{:-}", VulgarFraction::new(10, -3)));
/// assert_eq!("-¹⁄₀", format!("{:-}", VulgarFraction::new(-1, 0)));
/// assert_eq!("-⁰⁄₁", format!("{:-}", VulgarFraction::new(0, -1)));
///
/// // No single character fraction
/// assert_eq!("¹⁄₄", format!("{:#}", VulgarFraction::new(1, 4)));
/// ```
///
/// [Vulgar Fraction]: https://en.wikipedia.org/wiki/Fraction_(mathematics)#Simple,_common,_or_vulgar_fractions
/// [single character fractions]: http://unicodefractions.com
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VulgarFraction<T> {
    /// The number displayed above the fraction line.
    pub numerator: T,
    /// The number displayed below the fraction line.
    pub denominator: T,
}

impl<T> VulgarFraction<T> {
    /// Creates a new fraction from a numerator and denominator.
    pub const fn new(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl<T> From<(T, T)> for VulgarFraction<T> {
    fn from((numerator, denominator): (T, T)) -> Self {
        VulgarFraction {
            numerator,
            denominator,
        }
    }
}

impl<T> From<T> for VulgarFraction<T>
where
    T: Integer,
{
    fn from(value: T) -> Self {
        VulgarFraction::new(value, T::ONE)
    }
}

impl<T> fmt::Display for VulgarFraction<T>
where
    T: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (sign, numerator, denominator) = extract_sign(self.numerator, self.denominator, f);

        if let Some(sign) = sign {
            f.write_char(sign)?;
        }

        if let Some(frac) = (!f.alternate())
            .then(|| find_single_character_fraction(numerator, denominator))
            .flatten()
        {
            f.write_char(frac)
        } else {
            write!(f, "{}", Superscript(numerator))?;
            const FRACTION_SLASH: char = '\u{2044}';
            f.write_char(FRACTION_SLASH)?;
            write!(f, "{}", Subscript(denominator))
        }
    }
}

fn extract_sign<T>(numerator: T, denominator: T, f: &fmt::Formatter) -> (Option<char>, T, T)
where
    T: Integer,
{
    match numerator.sign() * denominator.sign() {
        Sign::Positive if f.sign_plus() => (Some('+'), numerator.abs(), denominator.abs()),
        Sign::Negative if f.sign_minus() => (Some('-'), numerator.abs(), denominator.abs()),
        _ => (None, numerator, denominator),
    }
}

fn find_single_character_fraction<N>(numerator: N, denominator: N) -> Option<char>
where
    N: TryInto<u8>,
{
    match (numerator.try_into().ok()?, denominator.try_into().ok()?) {
        (1u8, 4u8) => Some('\u{bc}'),
        (1u8, 2u8) => Some('\u{bd}'),
        (3u8, 4u8) => Some('\u{be}'),
        (1u8, 7u8) => Some('\u{2150}'),
        (1u8, 9u8) => Some('\u{2151}'),
        (1u8, 10u8) => Some('\u{2152}'),
        (1u8, 3u8) => Some('\u{2153}'),
        (2u8, 3u8) => Some('\u{2154}'),
        (1u8, 5u8) => Some('\u{2155}'),
        (2u8, 5u8) => Some('\u{2156}'),
        (3u8, 5u8) => Some('\u{2157}'),
        (4u8, 5u8) => Some('\u{2158}'),
        (1u8, 6u8) => Some('\u{2159}'),
        (5u8, 6u8) => Some('\u{215a}'),
        (1u8, 8u8) => Some('\u{215b}'),
        (3u8, 8u8) => Some('\u{215c}'),
        (5u8, 8u8) => Some('\u{215d}'),
        (7u8, 8u8) => Some('\u{215e}'),
        (0u8, 3u8) => Some('\u{2189}'),
        _ => None,
    }
}
