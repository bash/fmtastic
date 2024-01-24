use core::fmt;
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
#[allow(private_bounds)]
pub trait Integer: sealed::Integer_ {}

/// Abstraction over signed integer types.
pub trait SignedInteger: Integer {}

/// Abstraction over unsigned integer types.
/// Unsigned integers can be formatted as [`Segmented`](`crate::Segmented`) or [`TallyMarks`](`crate::TallyMarks`).
pub trait UnsignedInteger: Integer {}

#[derive(Debug)]
pub(crate) struct Ten;

#[derive(Debug)]
pub(crate) struct Two;

pub(crate) trait Base<I: sealed::Integer_>: fmt::Debug {
    const VALUE: I;
    fn ilog(x: I) -> u32;
}

mod sealed {
    use super::*;
    use std::ops::{Div, Rem, Sub};

    pub(crate) trait Integer_
    where
        Self: Copy,
        Self: Div<Self, Output = Self>,
        Self: Rem<Self, Output = Self>,
        Self: TryInto<u8>,
        Self: PartialOrd<Self>,
        Self: Sub<Self, Output = Self>,
        Self: std::fmt::Debug,
    {
        const ZERO: Self;
        const ONE: Self;
        const FIVE: Self;

        type BaseTwo: Base<Self>;
        type BaseTen: Base<Self>;

        fn range(from: Self, to: Self) -> impl Iterator<Item = Self> + DoubleEndedIterator;

        fn range_inclusive(
            from: Self,
            to: Self,
        ) -> impl Iterator<Item = Self> + DoubleEndedIterator;

        /// The sign of the integer value.
        /// Note that zero has a positive sign for our purposes.
        fn sign(self) -> Sign;

        fn abs(self) -> Self;

        fn as_usize(self) -> usize;

        fn pow(self, exp: u32) -> Self;
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
            const FIVE: Self = 5;

            type BaseTwo = Two;
            type BaseTen = Ten;

            fn range(from: Self, to: Self) -> impl Iterator<Item = Self> + DoubleEndedIterator {
                from..to
            }

            fn range_inclusive(
                from: Self,
                to: Self,
            ) -> impl Iterator<Item = Self> + DoubleEndedIterator {
                from..=to
            }

            fn as_usize(self) -> usize {
                self as usize
            }

            fn pow(self, exp: u32) -> Self {
                self.pow(exp)
            }
        };
    }

    macro_rules! impl_bases {
        ($ty:ty) => {
            impl Base<$ty> for Two {
                const VALUE: $ty = 2;

                fn ilog(x: $ty) -> u32 {
                    x.ilog2()
                }
            }

            impl Base<$ty> for Ten {
                const VALUE: $ty = 10;

                fn ilog(x: $ty) -> u32 {
                    x.ilog10()
                }
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

                impl_bases!($ty);
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

                impl_bases!($ty);
            )+
        };
    }

    impl_unsigned_integer!(u8, u16, u32, u64, usize);

    impl_signed_integer!(i8, i16, i32, i64, isize);
}
