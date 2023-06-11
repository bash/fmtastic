use crate::integer::{abs, sign, Sign};
use std::fmt::{self, Write};
use std::iter;

/// A number that can be formatted as superscript using the [`Display`][`std::fmt::Display`] trait.
///
/// [`Display`][`std::fmt::Display`] is implemented for all common number types.
///
/// ## Formatting Flags
/// ### Sign: `+`
/// Use the `+` flag to always include the + sign for positive numbers.
///
/// ## Examples
/// ```
/// # use fmtastic::Superscript;
/// assert_eq!("¹²³", format!("{}", Superscript(123)));
/// assert_eq!("⁰", format!("{}", Superscript(0)));
/// assert_eq!("⁻¹²³", format!("{}", Superscript(-123)));
/// assert_eq!("⁺¹²³", format!("{:+}", Superscript(123)));
///
/// // Binary
/// assert_eq!("¹⁰¹⁰¹⁰", format!("{:b}", Superscript(0b101010)));
/// assert_eq!("⁺¹⁰¹⁰¹⁰", format!("{:+b}", Superscript(0b101010)));
/// assert_eq!("⁻¹⁰¹⁰¹⁰", format!("{:b}", Superscript(-0b101010)));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Superscript<T>(pub T);

/// A number that can be formatted as subscript using the [`Display`][`std::fmt::Display`] trait.
///
/// [`Display`][`std::fmt::Display`] is implemented for all common number types.
///
/// ## Formatting Flags
/// ### Sign: `+`
/// Use the `+` flag to always include the + sign for positive numbers.
///
/// ## Examples
/// ```
/// # use fmtastic::Subscript;
/// assert_eq!("₁₂₃", format!("{}", Subscript(123)));
/// assert_eq!("₀", format!("{}", Subscript(0)));
/// assert_eq!("₋₁₂₃", format!("{}", Subscript(-123)));
/// assert_eq!("₊₁₂₃", format!("{:+}", Subscript(123)));
///
/// // Binary
/// assert_eq!("₁₀₁₀₁₀", format!("{:b}", Subscript(0b101010)));
/// assert_eq!("₊₁₀₁₀₁₀", format!("{:+b}", Subscript(0b101010)));
/// assert_eq!("₋₁₀₁₀₁₀", format!("{:b}", Subscript(-0b101010)));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Subscript<T>(pub T);

macro_rules! impl_fmt {
    ($trait:ident for $wrapper:ident<$($t:ident),+>, digits = $digits:expr, minus = $minus:expr, plus = $plus:expr) => {
        $(
            impl fmt::$trait for $wrapper<$t> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    const DIGITS: &[char] = &$digits;
                    const BASE: $t = DIGITS.len() as $t;
                    const LARGEST_EXPONTENT_OF_BASE: $t = {
                        let mut exponent: $t = 1;
                        while let Some(e) = exponent.checked_mul(BASE) {
                            exponent = e;
                        }
                        exponent
                    };

                    match sign(self.0) {
                        Sign::Positive if f.sign_plus() => f.write_char($plus)?,
                        Sign::Negative => f.write_char($minus)?,
                        _ => {},
                    };

                    let n = abs(self.0);

                    if (n == 0) {
                        f.write_char(DIGITS[0])
                    } else {
                        iter::successors(
                            Some((0, n, LARGEST_EXPONTENT_OF_BASE)),
                            |(_, n, div)| (*div != 0).then(|| (n / div, n % div, div / BASE)),
                        )
                        .map(|(digit, ..)| digit as usize)
                        .skip_while(|digit| *digit == 0)
                        .map(|digit| DIGITS[digit])
                        .map(|digit| f.write_char(digit))
                        .collect()
                    }
                }
            }
        )+
    }
}

impl_fmt!(
    Display for Superscript<i8, u8, i16, u16, i32, u32, i64, u64, usize, isize>,
    digits = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'], minus = '⁻', plus = '⁺');

impl_fmt!(
    Binary for Superscript<i8, u8, i16, u16, i32, u32, i64, u64, usize, isize>,
    digits = ['⁰', '¹'], minus = '⁻', plus = '⁺');

impl_fmt!(
    Display for Subscript<i8, u8, i16, u16, i32, u32, i64, u64, usize, isize>,
    digits = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'], minus = '₋', plus = '₊');

impl_fmt!(
    Binary for Subscript<i8, u8, i16, u16, i32, u32, i64, u64, usize, isize>,
    digits = ['₀', '₁'], minus = '₋', plus = '₊');

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_as_superscript() {
        for (expected, input) in [
            ("⁰", 0),
            ("¹", 1),
            ("²", 2),
            ("³", 3),
            ("⁴", 4),
            ("⁵", 5),
            ("⁶", 6),
            ("⁷", 7),
            ("⁸", 8),
            ("⁹", 9),
            ("¹⁰", 10),
            ("¹²³⁴⁵⁶⁷⁸⁹⁰", 1234567890),
            ("⁻¹²³⁴⁵⁶⁷⁸⁹⁰", -1234567890),
        ] {
            assert_eq!(expected, Superscript(input).to_string())
        }
    }

    #[test]
    fn adds_superscript_plus_sign_to_positive_numbers() {
        assert_eq!("⁺⁰", format!("{:+}", Superscript(0)));
        assert_eq!("⁺¹²³⁴⁵⁶⁷⁸⁹⁰", format!("{:+}", Superscript(1234567890)));
        assert_eq!("⁻¹²³⁴⁵⁶⁷⁸⁹⁰", format!("{:+}", Superscript(-1234567890)));
    }

    #[test]
    fn formats_as_subscript() {
        for (expected, input) in [
            ("₀", 0),
            ("₁", 1),
            ("₂", 2),
            ("₃", 3),
            ("₄", 4),
            ("₅", 5),
            ("₆", 6),
            ("₇", 7),
            ("₈", 8),
            ("₉", 9),
            ("₁₀", 10),
            ("₁₂₃₄₅₆₇₈₉₀", 1234567890),
            ("₋₁₂₃₄₅₆₇₈₉₀", -1234567890),
        ] {
            assert_eq!(expected, Subscript(input).to_string())
        }
    }

    #[test]
    fn adds_subscript_plus_sign_to_positive_numbers() {
        assert_eq!("₊₀", format!("{:+}", Subscript(0)));
        assert_eq!("₊₁₂₃₄₅₆₇₈₉₀", format!("{:+}", Subscript(1234567890)));
        assert_eq!("₋₁₂₃₄₅₆₇₈₉₀", format!("{:+}", Subscript(-1234567890)));
    }
}
