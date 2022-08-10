use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{One, Zero};

use crate::ring::{arithmetic::xgcd, Ring};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<'a> Ring<'a> for ModInt {
    fn is_field(&self) -> Option<bool> {
        self.is_field
    }

    fn is_commutative(&self) -> bool {
        true
    }

    fn inverse(&self) -> Option<Self> {
        let (gcd, _, inv) = xgcd(&self.value, &self.modulus);

        if gcd == 1 {
            return Some(Self::new(inv, self.modulus));
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
