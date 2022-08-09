use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num::{pow, One, Zero};

use crate::ring::{polynomial::Polynomial, Ring};

#[derive(Debug, Clone, PartialEq)]
pub struct UPolynomial<'a, R: Ring<'a>> {
    pub coefficients: Vec<R>,
    pub degree: usize,
    phantom: PhantomData<&'a R>,
}

impl<'a, R: Ring<'a>> UPolynomial<'a, R> {
    fn scale(mut self, scalar: R) -> Self {
        for c in self.coefficients.iter_mut() {
            *c = scalar.clone() * c.clone();
        }
        self
    }

    /// Use Horner's method to evaluate a polynomial at a given point
    fn eval(&self, point: R) -> R {
        let mut s = self.coefficients[self.degree].clone();
        for k in 1..(self.degree + 1) {
            s *= point.clone();
            s += self.coefficients[self.degree - k].clone()
        }
        s
    }
}

impl<'a, R: Ring<'a>> Add<Self> for UPolynomial<'a, R> {
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

impl<'a, R: Ring<'a>> Sub<Self> for UPolynomial<'a, R> {
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

impl<'a, R: Ring<'a>> Mul<Self> for UPolynomial<'a, R> {
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

impl<'a, R: Ring<'a>> Zero for UPolynomial<'a, R> {
    fn zero() -> Self {
        Self {
            coefficients: vec![R::zero()],
            degree: 0,
            phantom: PhantomData,
        }
    }
    fn is_zero(&self) -> bool {
        self.degree == 0 && self.coefficients[0] == R::zero()
    }
}

impl<'a, R: Ring<'a>> One for UPolynomial<'a, R> {
    fn one() -> Self {
        Self {
            coefficients: vec![R::one()],
            degree: 0,
            phantom: PhantomData,
        }
    }
    fn is_one(&self) -> bool {
        self.degree == 0 && self.coefficients[0] == R::one()
    }
}

impl<'a, R: Ring<'a>> Add<&'a Self> for UPolynomial<'a, R> {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self {
        let other = other.clone();
        self + other
    }
}

impl<'a, R: Ring<'a>> Sub<&'a Self> for UPolynomial<'a, R> {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self {
        let other = other.clone();
        self - other
    }
}
/*
impl<'a, R: Ring<'a>> Mul<&'a Self> for UPolynomial<'a, R> {
    type Output = Self;

    fn mul(mut self, other: &'a Self) -> Self {
        let mut other = other.clone();
        self * other
    }
}
*/
/*
impl<'a, R: Ring<'a>> Ring<'a> for UPolynomial<'a, R> {}

impl<'a, R: Ring<'a>> Polynomial<'a> for UPolynomial<'a, R> {
    type CoefRing = R;
    fn dim(&self) -> usize {
        1
    }
}*/
