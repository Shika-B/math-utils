pub mod polynomial;

use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use num::{BigInt, BigRational, BigUint, One, Zero};

pub trait Ring:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Zero<Output = Self>
    + One<Output = Self>
    + PartialEq
    + Debug
    + Clone
{
}

impl Ring for i8 {}
impl Ring for i16 {}
impl Ring for i32 {}
impl Ring for i64 {}
impl Ring for i128 {}
impl Ring for isize {}

impl Ring for u8 {}
impl Ring for u16 {}
impl Ring for u32 {}
impl Ring for u64 {}
impl Ring for u128 {}
impl Ring for usize {}

impl Ring for f32 {}
impl Ring for f64 {}

impl Ring for BigInt {}
impl Ring for BigUint {}
impl Ring for BigRational {}
