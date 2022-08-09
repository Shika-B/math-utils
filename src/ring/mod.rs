use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use num::{BigInt, BigRational, BigUint, One, Zero};

pub trait Ring: Add + Mul + Sub + Zero + One + Debug + Clone {}

pub trait Polynomial: Ring {
    type CoefRing: Ring;
    fn dim(&self) -> usize;
}

// Todo: Write a generic solution using macros
#[derive(Debug, Clone)]
struct UPolynomial<R: Ring> {
    pub coefficients: Vec<R>,
    pub degree: usize,
}
impl<R: Ring> Add<Self> for UPolynomial<R> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        if self.degree < other.degree {
            return other + self;
        }
        for (c1, c2) in self.coefficients.iter_mut().zip(other.coefficients) {
            *c1 = c1.clone() + c2;
        }
        return self;
    }
}

// impl<R: Ring> Ring for UPolynomial<R> {}

/*impl<R: Ring> Polynomial for UPolynomial<R> {
    type CoefRing = R;
    fn dim(&self) -> usize {
        1
    }
}*/

/*#[derive(Debug, Clone)]
struct MultiPolynomial<R: Ring> {
    /// number of indeterminates
    dim: usize,
    /// ordered in lexical order
    coefficients: Vec<R>,
}*/

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
