use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{Pod, Zeroable};
use fixed::types::I80F48;

/// A fixed-point number with 48 bits for the integer part and 48 bits for the fractional part.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Numeric {
    bits: [u8; 16],
}

impl Numeric {
    /// The zero value.
    pub const ZERO: Self = Numeric {
        bits: I80F48::ZERO.to_bits().to_le_bytes(),
    };

    /// Creates a new `Numeric` from a `u64`.
    pub fn from_u64(value: u64) -> Self {
        let i = I80F48::from_num(value);
        Self::from_i80f48(i)
    }

    /// Returns the `Numeric` as a `u64`.
    pub fn to_u64(&self) -> u64 {
        self.floor().to_i80f48().to_num::<u64>()
    }

    /// Creates a new `Numeric` from a fraction.
    pub fn from_fraction(numerator: u64, denominator: u64) -> Self {
        let f = I80F48::from_num(numerator) / I80F48::from_num(denominator);
        Self::from_i80f48(f)
    }

    /// Returns the floor of the `Numeric`.
    pub fn floor(&self) -> Self {
        Self::from_i80f48(self.to_i80f48().floor())
    }

    /// Creates a new `Numeric` from an `I80F48`.
    pub fn from_i80f48(value: I80F48) -> Self {
        Self {
            bits: value.to_bits().to_le_bytes(),
        }
    }

    /// Returns the `Numeric` as an `I80F48`.
    pub fn to_i80f48(&self) -> I80F48 {
        I80F48::from_bits(i128::from_le_bytes(self.bits))
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        Numeric::from_i80f48(self.to_i80f48() + other.to_i80f48())
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, other: Numeric) -> Numeric {
        Numeric::from_i80f48(self.to_i80f48() - other.to_i80f48())
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        Numeric::from_i80f48(self.to_i80f48() * other.to_i80f48())
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, other: Numeric) -> Numeric {
        Numeric::from_i80f48(self.to_i80f48() / other.to_i80f48())
    }
}

impl AddAssign for Numeric {
    fn add_assign(&mut self, other: Numeric) {
        *self = *self + other;
    }
}

impl SubAssign for Numeric {
    fn sub_assign(&mut self, other: Numeric) {
        *self = *self - other;
    }
}

impl MulAssign for Numeric {
    fn mul_assign(&mut self, other: Numeric) {
        *self = *self * other;
    }
}

impl DivAssign for Numeric {
    fn div_assign(&mut self, other: Numeric) {
        *self = *self / other;
    }
}

impl PartialEq for Numeric {
    fn eq(&self, other: &Self) -> bool {
        self.to_i80f48() == other.to_i80f48()
    }
}

impl PartialOrd for Numeric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_i80f48().partial_cmp(&other.to_i80f48())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_add() {
        let a = Numeric::from_i80f48(fixed::types::I80F48::from_num(5));
        let b = Numeric::from_i80f48(fixed::types::I80F48::from_num(3));
        let result = a + b;
        assert_eq!(result.to_i80f48(), fixed::types::I80F48::from_num(8));
    }

    #[test]
    fn test_numeric_sub() {
        let a = Numeric::from_i80f48(fixed::types::I80F48::from_num(5));
        let b = Numeric::from_i80f48(fixed::types::I80F48::from_num(3));
        let result = a - b;
        assert_eq!(result.to_i80f48(), fixed::types::I80F48::from_num(2));
    }

    #[test]
    fn test_numeric_mul() {
        let a = Numeric::from_i80f48(fixed::types::I80F48::from_num(5));
        let b = Numeric::from_i80f48(fixed::types::I80F48::from_num(3));
        let result = a * b;
        assert_eq!(result.to_i80f48(), fixed::types::I80F48::from_num(15));
    }

    #[test]
    fn test_numeric_div() {
        let a = Numeric::from_i80f48(fixed::types::I80F48::from_num(15));
        let b = Numeric::from_i80f48(fixed::types::I80F48::from_num(3));
        let result = a / b;
        assert_eq!(result.to_i80f48(), fixed::types::I80F48::from_num(5));
    }

    #[test]
    fn test_numeric_add_assign() {
        let mut a = Numeric::from_i80f48(fixed::types::I80F48::from_num(5));
        let b = Numeric::from_i80f48(fixed::types::I80F48::from_num(3));
        a += b;
        assert_eq!(a.to_i80f48(), fixed::types::I80F48::from_num(8));
    }

    #[test]
    fn test_numeric_comparison() {
        let a = Numeric::from_i80f48(fixed::types::I80F48::from_num(5));
        let b = Numeric::from_i80f48(fixed::types::I80F48::from_num(3));
        let c = Numeric::from_i80f48(fixed::types::I80F48::from_num(5));

        assert!(a > b);
        assert!(b < a);
        assert!(a >= c);
        assert!(c <= a);
        assert_eq!(a, c);
        assert!(a != b);
    }
}
