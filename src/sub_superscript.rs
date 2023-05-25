use std::fmt::{self, Write};
use std::iter;

/// A number that can be formatted as superscript using the [`Display`][`std::fmt::Display`] trait.
///
/// [`Display`][`std::fmt::Display`] is implemented for all common number types.
///
/// ## Flags
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
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Superscript<T>(pub T);

/// A number that can be formatted as subscript using the [`Display`][`std::fmt::Display`] trait.
///
/// [`Display`][`std::fmt::Display`] is implemented for all common number types.
///
/// ## Flags
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
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Subscript<T>(pub T);

macro_rules! impl_display {
    ($wrapper:ident<$($t:ident $($s:ident)?),+>, digits = $digits:expr, minus = $minus:expr, plus = $plus:expr) => {
        $(
            impl fmt::Display for $wrapper<$t> {
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

                    $(
                        signed!($s);

                        if self.0 < 0 {
                            f.write_char($minus)?;
                        } else if f.sign_plus() {
                            f.write_char($plus)?;
                        }
                    )?

                    #[allow(unused_mut)]
                    let mut n = self.0;
                    $(
                        signed!($s);
                        n = n.abs();
                    )?

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

macro_rules! signed {
    (signed) => {};
}

impl_display!(
    Superscript<i8 signed, u8, i16 signed, u16, i32 signed, u32, i64 signed, u64, usize, isize signed>,
    digits = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'], minus = '⁻', plus = '⁺');

impl_display!(
    Subscript<i8 signed, u8, i16 signed, u16, i32 signed, u32, i64 signed, u64, usize, isize signed>,
    digits = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'], minus = '₋', plus = '₊');

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_superscript() {
        assert_eq!("⁰", format!("{}", Superscript(0)));
        assert_eq!("⁺⁰", format!("{:+}", Superscript(0)));
        assert_eq!("¹", format!("{}", Superscript(1)));
        assert_eq!("²", format!("{}", Superscript(2)));
        assert_eq!("³", format!("{}", Superscript(3)));
        assert_eq!("⁴", format!("{}", Superscript(4)));
        assert_eq!("⁵", format!("{}", Superscript(5)));
        assert_eq!("⁶", format!("{}", Superscript(6)));
        assert_eq!("⁷", format!("{}", Superscript(7)));
        assert_eq!("⁸", format!("{}", Superscript(8)));
        assert_eq!("⁹", format!("{}", Superscript(9)));
        assert_eq!("¹⁰", format!("{}", Superscript(10)));
        assert_eq!("¹²³⁴⁵⁶⁷⁸⁹⁰", format!("{}", Superscript(1234567890)));
        assert_eq!("⁺¹²³⁴⁵⁶⁷⁸⁹⁰", format!("{:+}", Superscript(1234567890)));
        assert_eq!("⁻¹²³⁴⁵⁶⁷⁸⁹⁰", format!("{:}", Superscript(-1234567890)));
    }

    #[test]
    fn formats_subscript() {
        assert_eq!("₀", format!("{}", Subscript(0)));
        assert_eq!("₊₀", format!("{:+}", Subscript(0)));
        assert_eq!("₁", format!("{}", Subscript(1)));
        assert_eq!("₂", format!("{}", Subscript(2)));
        assert_eq!("₃", format!("{}", Subscript(3)));
        assert_eq!("₄", format!("{}", Subscript(4)));
        assert_eq!("₅", format!("{}", Subscript(5)));
        assert_eq!("₆", format!("{}", Subscript(6)));
        assert_eq!("₇", format!("{}", Subscript(7)));
        assert_eq!("₈", format!("{}", Subscript(8)));
        assert_eq!("₉", format!("{}", Subscript(9)));
        assert_eq!("₁₀", format!("{}", Subscript(10)));
        assert_eq!("₁₂₃₄₅₆₇₈₉₀", format!("{}", Subscript(1234567890)));
        assert_eq!("₊₁₂₃₄₅₆₇₈₉₀", format!("{:+}", Subscript(1234567890)));
        assert_eq!("₋₁₂₃₄₅₆₇₈₉₀", format!("{:}", Subscript(-1234567890)));
    }
}
