use crate::integer::Sign;
use crate::Integer;
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

impl<T> From<T> for Superscript<T>
where
    T: Integer,
{
    fn from(value: T) -> Self {
        Superscript(value)
    }
}

impl<T> fmt::Display for Superscript<T>
where
    T: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_number_with_base_and_digits(
            f,
            self.0,
            '⁺',
            '⁻',
            &['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'],
        )
    }
}

impl<T> fmt::Binary for Superscript<T>
where
    T: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_number_with_base_and_digits(f, self.0, '⁺', '⁻', &['⁰', '¹'])
    }
}

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

impl<T> From<T> for Subscript<T>
where
    T: Integer,
{
    fn from(value: T) -> Self {
        Subscript(value)
    }
}

impl<T> fmt::Display for Subscript<T>
where
    T: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_number_with_base_and_digits(
            f,
            self.0,
            '₊',
            '₋',
            &['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'],
        )
    }
}

impl<T> fmt::Binary for Subscript<T>
where
    T: Integer,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_number_with_base_and_digits(f, self.0, '₊', '₋', &['₀', '₁'])
    }
}

fn fmt_number_with_base_and_digits<T: Integer>(
    f: &mut fmt::Formatter<'_>,
    n: T,
    plus: char,
    minus: char,
    digits: &[char],
) -> fmt::Result {
    match n.sign() {
        Sign::Positive if f.sign_plus() => f.write_char(plus)?,
        Sign::Negative => f.write_char(minus)?,
        _ => {}
    };

    if n == T::ZERO {
        f.write_char(digits[0])
    } else {
        iter_digits(n, T::from_usize(digits.len()))
            .map(|digit| digits[digit])
            .try_for_each(|digit| f.write_char(digit))
    }
}

pub(crate) fn iter_digits<T: Integer>(n: T, base: T) -> impl Iterator<Item = usize> {
    let n = n.abs();
    let largest_exponent_of_base: T = {
        let mut exponent: T = T::ONE;
        while let Some(e) = exponent.checked_mul(base) {
            exponent = e;
        }
        exponent
    };
    iter::successors(
        Some((T::ZERO, n, largest_exponent_of_base)),
        move |(_, n, div)| (*div != T::ZERO).then(|| (*n / *div, *n % *div, *div / base)),
    )
    .map(|(digit, ..)| digit.as_usize())
    .skip_while(|digit| *digit == 0)
}

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
