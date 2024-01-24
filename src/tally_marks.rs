use crate::integer::IntegerImpl;
use crate::UnsignedInteger;
use core::fmt::{self, Write};

/// Formats an unsigned integer as tally marks.
///
/// You may need to install an extra font such as [Noto Sans Symbols 2]
/// since most other fonts do not support these digits.
///
/// [Noto Sans Symbols 2]: https://fonts.google.com/noto/specimen/Noto+Sans+Symbols+2
///
/// ```
/// use fmtastic::TallyMarks;
///
/// assert_eq!("", TallyMarks(0_u32).to_string());
/// assert_eq!("𝍷", TallyMarks(1_u32).to_string());
/// assert_eq!("𝍷𝍷", TallyMarks(2_u32).to_string());
/// assert_eq!("𝍷𝍷𝍷", TallyMarks(3_u32).to_string());
/// assert_eq!("𝍷𝍷𝍷𝍷", TallyMarks(4_u32).to_string());
/// assert_eq!("𝍸", TallyMarks(5_u32).to_string());
/// assert_eq!("𝍸𝍷", TallyMarks(6_u32).to_string());
/// assert_eq!("𝍸𝍸𝍸𝍷𝍷", TallyMarks(17_u32).to_string());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TallyMarks<T>(pub T);

impl<T> From<T> for TallyMarks<T>
where
    T: UnsignedInteger,
{
    fn from(value: T) -> Self {
        TallyMarks(value)
    }
}

impl<T> fmt::Display for TallyMarks<T>
where
    T: UnsignedInteger,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_tally_marks(self.0.to_impl(), f)
    }
}

fn fmt_tally_marks<T: IntegerImpl>(n: T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    const TALLY_MARK_ONE: char = '\u{1D377}';
    const TALLY_MARK_FIVE: char = '\u{1D378}';
    let (fives, ones) = (n / T::FIVE, n % T::FIVE);
    T::range(T::ZERO, fives).try_for_each(|_| f.write_char(TALLY_MARK_FIVE))?;
    T::range(T::ZERO, ones).try_for_each(|_| f.write_char(TALLY_MARK_ONE))?;
    Ok(())
}
