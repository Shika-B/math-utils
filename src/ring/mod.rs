pub mod mod_int;
pub mod polynomial;

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num::{traits::Inv, BigInt, BigRational, One, Rational32, Rational64, Zero};

pub use num::pow;

pub trait Ring<'a>:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + AddAssign<&'a Self>
    + SubAssign<&'a Self>
    + MulAssign<&'a Self>
    + Zero
    + One
    + Neg<Output = Self>
    + Add<&'a Self>
    + Sub<&'a Self>
    + Mul<&'a Self>
    + PartialEq
    + Debug
    + Clone
where
    Self: 'a,
{
    /// HACK: There are no type methods, so just use `R::zero().is_field()`
    fn is_field(&self) -> bool {
        false
    }

    /// HACK: There are no type methods, so just use `R::zero().is_commutative()`
    fn is_commutative(&self) -> bool {
        false
    }

    fn inverse(&self) -> Option<Self> {
        None
    }
}

impl<'a> Ring<'a> for i8 {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for i16 {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for i32 {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for i64 {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for i128 {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for isize {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for Rational32 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> bool {
        true
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}
impl<'a> Ring<'a> for Rational64 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> bool {
        true
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}

impl<'a> Ring<'a> for f32 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> bool {
        true
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}
impl<'a> Ring<'a> for f64 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> bool {
        true
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}

impl<'a> Ring<'a> for BigInt {
    fn is_commutative(&self) -> bool {
        true
    }
}

impl<'a> Ring<'a> for BigRational {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> bool {
        true
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}

// impl<'a> Ring<'a> for BigUint {}
/*
impl<'a> Ring<'a> for u8 {}
impl<'a> Ring<'a> for u16 {}
impl<'a> Ring<'a> for u32 {}
impl<'a> Ring<'a> for u64 {}
impl<'a> Ring<'a> for u128 {}
impl<'a> Ring<'a> for usize {}
*/
