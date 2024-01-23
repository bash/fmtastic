use crate::UnsignedInteger;
use std::fmt::{self, Write};

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
        const TALLY_MARK_ONE: char = '\u{1D377}';
        const TALLY_MARK_FIVE: char = '\u{1D378}';

        let five = T::from_usize(5);
        let (fives, ones) = (self.0 / five, self.0 % five);

        // We can't use `Range` here, that would require us
        // to be able to have `Step` as supertrait of `Integer`,
        // but `Step` is currently unstable.
        let mut iteration = T::ZERO;
        while iteration < fives {
            f.write_char(TALLY_MARK_FIVE)?;
            iteration += T::ONE;
        }

        for _ in 0..ones.as_usize() {
            f.write_char(TALLY_MARK_ONE)?;
        }

        Ok(())
    }
}
