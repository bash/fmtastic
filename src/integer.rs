use std::ops::Mul;

pub(crate) use self::sealed::*;

/// An abstraction over all integer types.
/// Integers can be formatted as [`Subscript`](crate::Subscript), [`Subscript`](crate::Subscript) or [`VulgarFraction`](crate::VulgarFraction).
///
/// **Do** use this trait if you want to abstract over integers that can be formatted
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
///
/// **Do not** depend on any of this trait's implementation details
/// such as its supertraits.
pub trait Integer: sealed::Integer_ {}

/// Abstraction over signed integer types.
pub trait SignedInteger: Integer {}

/// Abstraction over unsigned integer types.
/// Unsigned integers can be formatted as [`Segmented`](`crate::Segmented`) or [`TallyMarks`](`crate::TallyMarks`).
pub trait UnsignedInteger: Integer {}

mod sealed {
    use super::*;
    use std::ops::{AddAssign, Div, Rem};

    pub trait Integer_
    where
        Self: Copy,
        Self: PartialEq<Self>,
        Self: Div<Self, Output = Self>,
        Self: Rem<Self, Output = Self>,
        Self: 'static,
        Self: TryInto<u8>,
        Self: PartialOrd<Self>,
        Self: AddAssign<Self>,
    {
        const ZERO: Self;
        const ONE: Self;

        /// The sign of the integer value.
        /// Note that zero has a positive sign for our purposes.
        fn sign(self) -> Sign;

        fn abs(self) -> Self;

        fn checked_mul(self, rhs: Self) -> Option<Self>;

        fn as_usize(self) -> usize;

        fn from_usize(n: usize) -> Self;
    }

    pub enum Sign {
        None,
        Negative,
        Positive,
    }

    impl Mul for Sign {
        type Output = Sign;

        fn mul(self, rhs: Self) -> Self::Output {
            use Sign::*;
            match (self, rhs) {
                (None, _) | (_, None) => None,
                (Negative, Negative) => Positive,
                (Negative, Positive) => Negative,
                (Positive, Negative) => Negative,
                (Positive, Positive) => Positive,
            }
        }
    }

    macro_rules! common_integer_items {
        () => {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn checked_mul(self, rhs: Self) -> Option<Self> {
                self.checked_mul(rhs)
            }

            fn as_usize(self) -> usize {
                self as usize
            }

            fn from_usize(n: usize) -> Self {
                n as Self
            }
        };
    }

    macro_rules! impl_unsigned_integer {
        ($($ty:ty),+) => {
            $(
                impl Integer for $ty {}

                impl UnsignedInteger for $ty {}

                impl Integer_ for $ty {
                    common_integer_items!();

                    fn sign(self) -> Sign {
                        Sign::None
                    }

                    fn abs(self) -> Self {
                        self
                    }
                }
            )+
        };
    }

    macro_rules! impl_signed_integer {
        ($($ty:ty),+) => {
            $(
                impl Integer for $ty {}

                impl SignedInteger for $ty {}

                impl Integer_ for $ty {
                    common_integer_items!();

                    fn sign(self) -> Sign {
                        if self >= 0 {
                            Sign::Positive
                        } else {
                            Sign::Negative
                        }
                    }

                    fn abs(self) -> Self {
                        self.abs()
                    }
                }
            )+
        };
    }

    impl_unsigned_integer!(u8, u16, u32, u64, usize);

    impl_signed_integer!(i8, i16, i32, i64, isize);
}
