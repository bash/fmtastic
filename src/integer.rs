use core::fmt;
use core::ops::{Div, Rem, Sub};
use core::ops::{Mul, SubAssign};

pub(crate) trait IntegerImpl
where
    Self: Copy,
    Self: Div<Self, Output = Self>,
    Self: Rem<Self, Output = Self>,
    Self: TryInto<u8>,
    Self: TryFrom<u16>,
    Self: PartialOrd<Self>,
    Self: Sub<Self, Output = Self>,
    Self: SubAssign<Self>,
{
    const ZERO: Self;
    const ONE: Self;
    const FIVE: Self;

    type Public: crate::Integer;
    type BaseTwo: Base<Self>;
    type BaseTen: Base<Self>;
    type BaseSixteen: Base<Self>;

    fn range(from: Self, to: Self) -> impl DoubleEndedIterator<Item = Self>;

    fn sign(self) -> Sign {
        if self >= Self::ZERO {
            Sign::PositiveOrZero
        } else {
            Sign::Negative
        }
    }

    fn abs(self) -> Self;

    fn as_usize(self) -> usize;

    fn pow(self, exp: u32) -> Self;

    fn into_public(self) -> Self::Public;
}

#[allow(dead_code)] // This is clearly used dear compiler
pub(crate) trait UnsignedIntegerImpl: IntegerImpl + crate::roman::RomanInteger {}

pub(crate) enum Sign {
    Negative,
    PositiveOrZero,
}

impl Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self::Output {
        use Sign::*;
        match (self, rhs) {
            (Negative, Negative) => PositiveOrZero,
            (Negative, PositiveOrZero) => Negative,
            (PositiveOrZero, Negative) => Negative,
            (PositiveOrZero, PositiveOrZero) => PositiveOrZero,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Ten;

#[derive(Debug)]
pub(crate) struct Two;

#[derive(Debug)]
pub(crate) struct Sixteen;

pub(crate) trait Base<I: IntegerImpl>: fmt::Debug {
    const VALUE: I;

    fn ilog(x: I) -> u32;

    fn powers(x: I) -> impl Iterator<Item = I> {
        let largest_exp = if x == I::ZERO { 0 } else { Self::ilog(x) };
        (0..=largest_exp).rev().map(|e| Self::VALUE.pow(e))
    }
}

macro_rules! common_integer_items {
    ($ty:ty) => {
        const ZERO: Self = 0;
        const ONE: Self = 1;
        const FIVE: Self = 5;

        type Public = $ty;
        type BaseTwo = Two;
        type BaseTen = Ten;
        type BaseSixteen = Sixteen;

        fn range(from: Self, to: Self) -> impl DoubleEndedIterator<Item = Self> {
            from..to
        }

        fn as_usize(self) -> usize {
            self as usize
        }

        fn pow(self, exp: u32) -> Self {
            self.pow(exp)
        }

        fn into_public(self) -> Self::Public {
            self
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

        impl Base<$ty> for Sixteen {
            const VALUE: $ty = 16;

            fn ilog(x: $ty) -> u32 {
                x.ilog(Self::VALUE)
            }
        }
    };
}

macro_rules! impl_unsigned_integer {
    ($($ty:ty),+) => {
        $(
            impl crate::Integer for $ty {}
            impl crate::UnsignedInteger for $ty {}

            impl crate::ToIntegerImpl for $ty {
                type Impl = $ty;

                fn into_impl(self) -> $ty {
                    self
                }
            }

            impl crate::ToUnsignedIntegerImpl for $ty {
                type UnsignedImpl = $ty;
            }

            impl UnsignedIntegerImpl for $ty {}

            impl IntegerImpl for $ty {
                common_integer_items!($ty);

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
            impl crate::Integer for $ty {}
            impl crate::SignedInteger for $ty {}

            impl crate::ToIntegerImpl for $ty {
                type Impl = $ty;

                fn into_impl(self) -> $ty {
                    self
                }
            }

            impl IntegerImpl for $ty {
                common_integer_items!($ty);

                fn abs(self) -> Self {
                    self.abs()
                }
            }

            impl_bases!($ty);
        )+
    };
}

impl_unsigned_integer!(u8, u16, u32, u64, u128, usize);

impl_signed_integer!(i8, i16, i32, i64, i128, isize);
