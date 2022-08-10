use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModInt {
    value: i128,
    modulus: i128,
}

impl ModInt {
    pub fn new(value: i128, modulus: i128) -> Self {
        Self { value, modulus }
    }
    fn sanitize_mod(&mut self, other: &mut Self) {
        if (self.modulus == -1) ^ (other.modulus == -1) {
            let actual_modulus = self.modulus.max(other.modulus);
            self.modulus = actual_modulus;
            other.modulus = actual_modulus;
        } else {
            debug_assert_eq!(self.modulus, other.modulus);
        }
    }
}

impl Add<Self> for ModInt {
    type Output = Self;
    fn add(mut self, mut other: Self) -> Self {
        self.sanitize_mod(&mut other);
        Self {
            value: self.value + other.value % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl Sub<Self> for ModInt {
    type Output = Self;
    fn sub(mut self, mut other: Self) -> Self {
        self.sanitize_mod(&mut other);
        Self {
            value: self.value - other.value % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl Mul<Self> for ModInt {
    type Output = Self;
    fn mul(mut self, mut other: Self) -> Self {
        self.sanitize_mod(&mut other);
        Self {
            value: self.value * other.value % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl Zero for ModInt {
    fn is_zero(&self) -> bool {
        self.value == 0
    }
    fn zero() -> Self {
        Self {
            value: 0,
            modulus: -1, // Yes this is a hack and I need to keep track of it in every arithmetical operations,
                         // but I can't encode the modulus at the type-level anyway so..
        }
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
