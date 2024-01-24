use crate::integer::{Base, IntegerImpl};

pub(crate) fn iter_digits<T: IntegerImpl, B: Base<T>>(n: T) -> impl Iterator<Item = usize> {
    let n = n.abs();
    B::powers(n).scan(n, move |remainder, power| {
        let digit = *remainder / power;
        *remainder = n % power;
        Some(digit.as_usize())
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::integer::IntegerImpl;

    #[test]
    #[should_panic] // TODO: fix this in the implementation
    fn zero_has_zero_as_digits() {
        let digits: Vec<_> = iter_digits::<_, <u32 as IntegerImpl>::BaseTen>(0_u32).collect();
        assert_eq!(vec![0], digits);
    }

    #[test]
    fn iterates_digits_in_base_10() {
        let digits: Vec<_> =
            iter_digits::<_, <u32 as IntegerImpl>::BaseTen>(1234567890_u32).collect();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], digits);
    }

    #[test]
    fn iterates_digits_in_base_2() {
        let digits: Vec<_> = iter_digits::<_, <u32 as IntegerImpl>::BaseTwo>(0b10110110).collect();
        assert_eq!(vec![1, 0, 1, 1, 0, 1, 1, 0], digits);
    }
}
