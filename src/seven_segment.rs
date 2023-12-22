use crate::UnsignedInteger;
use std::fmt;
use std::ops::Range;

/// Formats an unsigned integer using seven-segment digits
/// from the [Legacy Computing] block.
///
/// You may need to install an extra font such as [Sieben 7] or [Noto Sans Symbols 2]
/// since most other fonts do not support these digits.
///
/// [Legacy Computing]: https://www.unicode.org/charts/PDF/U1FB00.pdf
/// [Sieben 7]: https://github.com/bash/sieben-7
/// [Noto Sans Symbols 2]: https://fonts.google.com/noto/specimen/Noto+Sans+Symbols+2
///
/// ```
/// use fmtastic::Segmented;
///
/// assert_eq!("🯶🯲🯸", Segmented(628_u32).to_string());
///
/// assert_eq!("🯰", Segmented(0_u32).to_string());
/// assert_eq!("🯱", Segmented(1_u32).to_string());
/// assert_eq!("🯲", Segmented(2_u32).to_string());
/// assert_eq!("🯳", Segmented(3_u32).to_string());
/// assert_eq!("🯴", Segmented(4_u32).to_string());
/// assert_eq!("🯵", Segmented(5_u32).to_string());
/// assert_eq!("🯶", Segmented(6_u32).to_string());
/// assert_eq!("🯷", Segmented(7_u32).to_string());
/// assert_eq!("🯸", Segmented(8_u32).to_string());
/// assert_eq!("🯹", Segmented(9_u32).to_string());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Segmented<T>(pub T);

impl<T> From<T> for Segmented<T>
where
    T: UnsignedInteger,
{
    fn from(value: T) -> Self {
        Segmented(value)
    }
}

impl<T> fmt::Display for Segmented<T>
where
    T: UnsignedInteger,
    Range<T>: Iterator<Item = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == T::ZERO {
            write!(f, "{}", DIGITS[0])?;
        } else {
            for digit in crate::sub_superscript::iter_digits(self.0, T::from_usize(DIGITS.len())) {
                write!(f, "{}", DIGITS[digit])?;
            }
        }

        Ok(())
    }
}

const DIGITS: [&str; 10] = [
    "\u{1FBF0}",
    "\u{1FBF1}",
    "\u{1FBF2}",
    "\u{1FBF3}",
    "\u{1FBF4}",
    "\u{1FBF5}",
    "\u{1FBF6}",
    "\u{1FBF7}",
    "\u{1FBF8}",
    "\u{1FBF9}",
];
