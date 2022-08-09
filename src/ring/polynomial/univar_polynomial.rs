use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use num::{One, Zero};

use crate::ring::{polynomial::Polynomial, Ring};

#[derive(Debug, Clone, PartialEq)]
pub struct UPolynomial<R: Ring> {
    pub coefficients: Vec<R>,
    pub degree: isize,
}

impl<R: Ring> Add<Self> for UPolynomial<R> {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {
        if self.degree < other.degree {
            std::mem::swap(&mut self, &mut other);
        }
        for (c1, c2) in self.coefficients.iter_mut().zip(other.coefficients) {
            *c1 = c1.clone() + c2;
        }
        return self;
    }
}

impl<R: Ring> Sub<Self> for UPolynomial<R> {
    type Output = Self;

    fn sub(mut self, mut other: Self) -> Self {
        if self.degree < other.degree {
            std::mem::swap(&mut self, &mut other);
        }
        for (c1, c2) in self.coefficients.iter_mut().zip(other.coefficients) {
            *c1 = c1.clone() - c2;
        }
        return self;
    }
}

impl<R: Ring> Mul<Self> for UPolynomial<R> {
    type Output = Self;

    fn mul(mut self, mut other: Self) -> Self {
        if self.degree < other.degree {
            std::mem::swap(&mut self, &mut other);
        }
        for (c1, c2) in self.coefficients.iter_mut().zip(other.coefficients) {
            *c1 = c1.clone() * c2;
        }
        return self;
    }
}
impl<R: Ring> Zero for UPolynomial<R> {
    fn zero() -> Self {
        Self {
            coefficients: vec![R::zero()],
            degree: 0,
        }
    }
    fn is_zero(&self) -> bool {
        self.degree == 0 && self.coefficients[0] == R::zero()
    }
}

impl<R: Ring> One for UPolynomial<R> {
    fn one() -> Self {
        Self {
            coefficients: vec![R::one()],
            degree: 0,
        }
    }
    fn is_one(&self) -> bool {
        self.degree == 0 && self.coefficients[0] == R::one()
    }
}

impl<R: Ring> Ring for UPolynomial<R> {}

impl<R: Ring> Polynomial for UPolynomial<R> {
    type CoefRing = R;
    fn dim(&self) -> usize {
        1
    }
}
