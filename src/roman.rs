// Adapted from Yann Villessuzanne's roman.rs under the
// Unlicense, at https://github.com/linfir/roman.rs/

use crate::integer::IntegerImpl;
use crate::UnsignedInteger;
use core::fmt;

/// Formats unsigned integers as Roman numerals.
///
/// By default, the dedicated unicode symbols for Roman numerals are used.
/// You can use [`Roman::ascii`] to use ASCII symbols instead.
///
/// ```
/// # use fmtastic::Roman;
/// assert_eq!("ⅾⅽⅽⅼⅹⅹⅹⅰⅹ", format!("{:#}", Roman::new(789_u16).unwrap())); // lowercase
/// assert_eq!("ⅯⅯⅩⅩⅠⅤ", format!("{}", Roman::new(2024_u16).unwrap()));
/// assert_eq!("MMXXIV", format!("{}", Roman::new(2024_u16).unwrap().ascii())); // ascii
/// assert_eq!("ⅠⅠⅠ", format!("{}", Roman::from(3_u8))); // u8's can always be formatted as Roman numeral
/// ```
///
/// ## Formatting Flags
/// ### Alternate `#`
/// By default uppercase numerals are used.
/// The alternate flag `#` can be used to switch to lowercase numerals.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Roman<T>(T, SymbolRepertoire);

impl<T> Roman<T> {
    /// Uses ASCII symbols instead of the dedicated unciode
    /// symbols for Roman numerals.
    pub fn ascii(mut self) -> Self {
        self.1 = SymbolRepertoire::Ascii;
        self
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
enum SymbolRepertoire {
    Unicode,
    Ascii,
}

impl From<u8> for Roman<u8> {
    fn from(value: u8) -> Self {
        Roman(value, SymbolRepertoire::Unicode)
    }
}

impl<T> Roman<T>
where
    T: UnsignedInteger,
{
    /// Creates a new [`Roman`] numeral.
    /// Returns `None` if the value is not between 1 and 3999.
    pub fn new(value: T) -> Option<Roman<T>> {
        if T::Impl::ZERO < value.into_impl() && value.into_impl() <= T::UnsignedImpl::ROMAN_MAX {
            Some(Roman(value, SymbolRepertoire::Unicode))
        } else {
            None
        }
    }
}

impl<T> fmt::Display for Roman<T>
where
    T: UnsignedInteger,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut n = self.0.into_impl();
        for (symbol, value) in roman_pairs::<T>(self.1, f.alternate()) {
            let value = value.into_impl();
            while n >= value {
                n -= value;
                write!(f, "{symbol}")?;
            }
        }
        debug_assert!(n == T::Impl::ZERO);
        Ok(())
    }
}

fn roman_pairs<T>(
    repertoire: SymbolRepertoire,
    lowercase: bool,
) -> impl Iterator<Item = (&'static str, T)>
where
    T: UnsignedInteger,
{
    ROMAN_PAIRS.iter().copied().filter_map(
        move |(upper_unicode, lower_unicode, upper_ascii, lower_ascii, value)| {
            let symbol = match (repertoire, lowercase) {
                (SymbolRepertoire::Unicode, false) => upper_unicode,
                (SymbolRepertoire::Unicode, true) => lower_unicode,
                (SymbolRepertoire::Ascii, false) => upper_ascii,
                (SymbolRepertoire::Ascii, true) => lower_ascii,
            };
            Some((symbol, T::Impl::try_from(value).ok()?.into_public()))
        },
    )
}

static ROMAN_PAIRS: &[(&str, &str, &str, &str, u16)] = &[
    ("Ⅿ", "ⅿ", "M", "m", 1000),
    ("ⅭⅯ", "ⅽⅿ", "CM", "cm", 900),
    ("Ⅾ", "ⅾ", "D", "d", 500),
    ("ⅭⅮ", "ⅽⅾ", "CD", "cd", 400),
    ("Ⅽ", "ⅽ", "C", "c", 100),
    ("ⅩⅭ", "ⅹⅽ", "XC", "xc", 90),
    ("Ⅼ", "ⅼ", "L", "l", 50),
    ("ⅩⅬ", "ⅹⅼ", "XL", "xl", 40),
    ("Ⅹ", "ⅹ", "X", "x", 10),
    ("ⅠⅩ", "ⅰⅹ", "IX", "ix", 9),
    ("Ⅴ", "ⅴ", "V", "v", 5),
    ("ⅠⅤ", "ⅰⅴ", "IV", "iv", 4),
    ("Ⅰ", "ⅰ", "I", "i", 1),
];

pub(crate) trait RomanInteger {
    const ROMAN_MAX: Self;
}

impl RomanInteger for u8 {
    const ROMAN_MAX: Self = u8::MAX;
}

macro_rules! impl_roman_integer {
    ($($ty:ty),*) => {
        $(
            impl RomanInteger for $ty {
                /// The largest number representable as a roman numeral.
                const ROMAN_MAX: Self = 3999;
            }
        )*
    }
}

impl_roman_integer!(u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_roman() {
        let roman =
            "I II III IV V VI VII VIII IX X XI XII XIII XIV XV XVI XVII XVIII XIX XX XXI XXII"
                .split(' ');
        for (i, x) in roman.enumerate() {
            let n = i + 1;
            assert_eq!(format!("{}", Roman::new(n).unwrap().ascii()), x);
        }
        assert_eq!(
            format!("{}", Roman::new(1984u32).unwrap().ascii()),
            "MCMLXXXIV"
        );
        assert_eq!(
            format!("{}", Roman::new(448u32).unwrap().ascii()),
            "CDXLVIII"
        );
    }
}
