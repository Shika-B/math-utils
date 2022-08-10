use crate::ring::Ring;

use num::{traits::Inv, BigInt, BigRational, Rational32, Rational64};

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
    fn is_field(&self) -> Option<bool> {
        Some(true)
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}
impl<'a> Ring<'a> for Rational64 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> Option<bool> {
        Some(true)
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}

impl<'a> Ring<'a> for f32 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> Option<bool> {
        Some(true)
    }
    fn inverse(&self) -> Option<Self> {
        Some(self.inv())
    }
}
impl<'a> Ring<'a> for f64 {
    fn is_commutative(&self) -> bool {
        true
    }
    fn is_field(&self) -> Option<bool> {
        Some(true)
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
    fn is_field(&self) -> Option<bool> {
        Some(true)
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
