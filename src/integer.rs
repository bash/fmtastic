use std::ops::Mul;

pub(crate) fn abs<T: Integer>(value: T) -> T {
    value.abs()
}

pub(crate) fn sign<T: Integer>(value: T) -> Sign {
    value.sign()
}

pub(crate) trait Integer
where
    Self: Copy,
{
    /// The sign of the integer value.
    /// Note that zero has a positive sign for our purposes.
    fn sign(self) -> Sign;

    fn abs(self) -> Self;
}

pub(crate) enum Sign {
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

macro_rules! impl_unsigned_integer {
    ($($ty:ty),+) => {
        $(
            impl Integer for $ty {
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
            impl Integer for $ty {
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
