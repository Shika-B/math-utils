pub mod polynomial;

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num::{BigInt, BigRational, One, Rational32, Rational64, Zero};

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
}

impl<'a> Ring<'a> for i8 {}
impl<'a> Ring<'a> for i16 {}
impl<'a> Ring<'a> for i32 {}
impl<'a> Ring<'a> for i64 {}
impl<'a> Ring<'a> for i128 {}
impl<'a> Ring<'a> for isize {}

/*
impl<'a> Ring<'a> for u8 {}
impl<'a> Ring<'a> for u16 {}
impl<'a> Ring<'a> for u32 {}
impl<'a> Ring<'a> for u64 {}
impl<'a> Ring<'a> for u128 {}
impl<'a> Ring<'a> for usize {}
*/

impl<'a> Ring<'a> for Rational32 {}
impl<'a> Ring<'a> for Rational64 {}

impl<'a> Ring<'a> for f32 {}
impl<'a> Ring<'a> for f64 {}

impl<'a> Ring<'a> for BigInt {}
// impl<'a> Ring<'a> for BigUint {}
impl<'a> Ring<'a> for BigRational {}
