use crate::digits::iter_digits;
use crate::integer::{Base, IntegerImpl};
use crate::UnsignedInteger;
use core::fmt;

/// Formats an unsigned integer using outlined digits
/// from the [Legacy Computing Supplement] block.
///
/// You may need to install an extra font such as [Kreative Square] to display these digits.
///
/// [Legacy Computing Supplement]: https://www.unicode.org/charts/PDF/U1CC00.pdf
/// [Kreative Square]: http://www.kreativekorp.com/software/fonts/ksquare/
///
/// ```
/// use fmtastic::Outlined;
///
/// assert_eq!("𜳶𜳲𜳸", Outlined(628_u32).to_string());
///
/// assert_eq!("𜳰", Outlined(0_u32).to_string());
/// assert_eq!("𜳱", Outlined(1_u32).to_string());
/// assert_eq!("𜳲", Outlined(2_u32).to_string());
/// assert_eq!("𜳳", Outlined(3_u32).to_string());
/// assert_eq!("𜳴", Outlined(4_u32).to_string());
/// assert_eq!("𜳵", Outlined(5_u32).to_string());
/// assert_eq!("𜳶", Outlined(6_u32).to_string());
/// assert_eq!("𜳷", Outlined(7_u32).to_string());
/// assert_eq!("𜳸", Outlined(8_u32).to_string());
/// assert_eq!("𜳹", Outlined(9_u32).to_string());
///
/// // Binary
/// assert_eq!("𜳰", format!("{:b}", Outlined(0_u8)));
/// assert_eq!("𜳱𜳰𜳱𜳰𜳱𜳰", format!("{:+b}", Outlined(0b101010_u8)));
///
/// // Hexadecimal
/// assert_eq!("𜳱𜳘𜳘𜳛𜳰", format!("{:X}", Outlined(0x1CCF0_u32)));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Outlined<T>(pub T);

impl<T> From<T> for Outlined<T>
where
    T: UnsignedInteger,
{
    fn from(value: T) -> Self {
        Outlined(value)
    }
}

impl<T> fmt::Binary for Outlined<T>
where
    T: UnsignedInteger,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_outlined::<_, <T::Impl as IntegerImpl>::BaseTwo>(self.0.into_impl(), f)
    }
}

impl<T> fmt::Display for Outlined<T>
where
    T: UnsignedInteger,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_outlined::<_, <T::Impl as IntegerImpl>::BaseTen>(self.0.into_impl(), f)
    }
}

impl<T> fmt::UpperHex for Outlined<T>
where
    T: UnsignedInteger,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_outlined::<_, <T::Impl as IntegerImpl>::BaseSixteen>(self.0.into_impl(), f)
    }
}

fn fmt_outlined<T: IntegerImpl, B: Base<T>>(n: T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    iter_digits::<_, B>(n).try_for_each(|digit| write!(f, "{}", DIGITS[digit]))
}

const DIGITS: [&str; 16] = [
    // Outlined digits 0-9
    "\u{1CCF0}",
    "\u{1CCF1}",
    "\u{1CCF2}",
    "\u{1CCF3}",
    "\u{1CCF4}",
    "\u{1CCF5}",
    "\u{1CCF6}",
    "\u{1CCF7}",
    "\u{1CCF8}",
    "\u{1CCF9}",
    // Outlined uppercase Latin alphabet A-F
    "\u{1CCD6}",
    "\u{1CCD7}",
    "\u{1CCD8}",
    "\u{1CCD9}",
    "\u{1CCDA}",
    "\u{1CCDB}",
];
