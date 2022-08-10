use std::{
    cmp::PartialEq,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num::{One, Zero};

use crate::ring::{arithmetic::xgcd, Ring};

#[derive(Debug, Clone, Copy)]
pub struct ModInt {
    value: i128,
    modulus: i128,
    is_field: Option<bool>,
}

impl ModInt {
    pub fn new(value: i128, modulus: i128) -> Self {
        Self {
            value,
            modulus,
            is_field: None,
        }
    }

    /// !!! Don't use this method if you don't know for sure `modulus` is prime !!!
    pub fn new_field(value: i128, modulus: i128) -> Self {
        Self {
            value,
            modulus,
            is_field: Some(true),
        }
    }

    pub fn mod_pow(&self, exp: i128) -> Self {
        if exp < 0 {
            match self.inverse() {
                Some(inv) => return inv.mod_pow(-exp),
                None => {
                    panic!("Cannot compute negative exponent of a non-inversible modular integer")
                }
            }
        } else if self.modulus == -1 {
            return *self;
        }
        let mut result = 0;
        let mut mask = 1i128;
        let mut pow2 = self.value;
        for _ in 0..127 {
            if mask > exp {
                break;
            }
            let is_set = (exp & mask) != 0;
            if is_set {
                result += pow2;
                result %= self.modulus
            }
            mask <<= 1;
            pow2 = pow2 * pow2;
            pow2 %= self.modulus;
        }
        Self::new(result, self.modulus)
    }

    /// Method only used in the arithemtical operations definition, to work around the fact that the `Zero` and `One` traits
    /// require universal (that is, not depending on the modulus) `zero` and `one` method.
    /// The workaround is to set the modulus to `-1` in these method and then check if the modulus is equal
    fn sanitize_mod(&mut self, other: &mut Self) {
        if (self.modulus == -1) || (other.modulus == -1) {
            let actual_modulus = self.modulus.max(other.modulus);
            self.modulus = actual_modulus;
            other.modulus = actual_modulus;
        } else {
            debug_assert_eq!(self.modulus, other.modulus);
        }
    }
}

impl PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        if (self.modulus == -1) || (other.modulus == -1) {
            return self.value == other.value;
        } else {
            debug_assert_eq!(self.modulus, other.modulus);
        }
        (self.value - other.value).rem_euclid(self.modulus) == 0
    }
}

impl<'a> Ring<'a> for ModInt {
    fn is_field(&self) -> Option<bool> {
        self.is_field
    }

    fn is_commutative(&self) -> bool {
        true
    }

    fn inverse(&self) -> Option<Self> {
        let (gcd, inv, _) = xgcd(&self.value, &self.modulus);

        if gcd == 1 {
            return Some(Self::new(inv.rem_euclid(self.modulus), self.modulus));
        } else {
            return None;
        }
    }
}

impl Add<Self> for ModInt {
    type Output = Self;
    fn add(mut self, mut other: Self) -> Self {
        self.sanitize_mod(&mut other);
        Self::new(self.value + other.value % self.modulus, self.modulus)
    }
}

impl Sub<Self> for ModInt {
    type Output = Self;
    fn sub(mut self, mut other: Self) -> Self {
        self.sanitize_mod(&mut other);
        Self::new(self.value - other.value % self.modulus, self.modulus)
    }
}

impl Mul<Self> for ModInt {
    type Output = Self;
    fn mul(mut self, mut other: Self) -> Self {
        self.sanitize_mod(&mut other);
        Self::new(self.value * other.value % self.modulus, self.modulus)
    }
}

impl Neg for ModInt {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.value = -1;
        self
    }
}

impl Zero for ModInt {
    fn is_zero(&self) -> bool {
        self.value == 0
    }
    fn zero() -> Self {
        Self::new(0, -1) // Yes this is a hack and I need to keep track of it in every arithmetical operations,
                         // but I can't encode the modulus at the type-level anyway so..
    }
}

impl One for ModInt {
    fn is_one(&self) -> bool {
        self.value == 0
    }
    fn one() -> Self {
        Self::new(1, -1) // Yes this is a hack and I need to keep track of it in every arithmetical operations,
                         // but I can't encode the modulus at the type-level anyway so..
    }
}

impl AddAssign<Self> for ModInt {
    fn add_assign(&mut self, mut other: Self) {
        self.sanitize_mod(&mut other);
        self.value += other.value;
    }
}

impl SubAssign<Self> for ModInt {
    fn sub_assign(&mut self, mut other: Self) {
        self.sanitize_mod(&mut other);
        self.value -= other.value;
    }
}

impl MulAssign<Self> for ModInt {
    fn mul_assign(&mut self, mut other: Self) {
        self.sanitize_mod(&mut other);
        self.value *= other.value;
    }
}
/*
macro_rules! ref_bin_op {
    ($trait:tt, $method_name:ident) => {
        impl<'a> $trait<&'a Self> for ModInt {
            type Output = Self;
            fn $method_name(mut self, other: &'a Self) -> Self {
                let mut other = other.clone();
                self.sanitize_mod(&mut other);
                $trait::$method_name(self, other)
            }
        }
    };
}

macro_rules! ref_assign_bin_op {
    ($trait:tt, $method_name:ident) => {
        impl<'a> $trait<&'a Self> for ModInt {
            fn $method_name(&mut self, other: &'a Self) {
                let mut other = other.clone();
                self.sanitize_mod(&mut other);
                $trait::$method_name(&mut self.value, other.value)
            }
        }
    };
}

ref_bin_op!(Add, add);
ref_bin_op!(Sub, sub);
ref_bin_op!(Mul, mul);
ref_assign_bin_op!(AddAssign, add_assign);
ref_assign_bin_op!(SubAssign, sub_assign);
ref_assign_bin_op!(MulAssign, mul_assign);
*/

mod test {
    use crate::ring::{ringed, Ring, Ringed};

    use super::ModInt;

    #[test]
    fn arithmetic() {
        let (x, y, z) = (ModInt::new(3, 21), ModInt::new(5, 21), ModInt::new(23, 21));
        let expected = ModInt::new(15, 21);
        assert_eq!(x * (y - z) + ringed::<ModInt, _>(3i32) * z, expected);
    }

    #[test]
    fn inverse() {
        let x = ModInt::new(5, 7);
        assert_eq!(x.inverse(), Some(ModInt::new(3, 7)))
    }

    #[test]
    fn mod_pow_test() {
        let x = ModInt::new(5, 7);

        let expected = ModInt::new(4, 7);
        assert_eq!(x.mod_pow(2), expected);

        let expected = ModInt::new(2, 7);
        assert_eq!(x.mod_pow(-2), expected)
    }
}
