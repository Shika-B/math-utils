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
    fn new(coefficients: Vec<R>) -> Self {
        Self {
            degree: coefficients.len() - 1,
            coefficients,
            phantom: PhantomData,
        }
    }
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
            println!("c1: {:#?}, c2: {:#?}", c1, c2);
            *c1 = c1.clone() + c2;
        }
        return self;
    }
}

impl<'a, R: Ring<'a>> Sub<Self> for UPolynomial<'a, R> {
    type Output = Self;

    fn sub(mut self, mut other: Self) -> Self {
        let a = if self.degree < other.degree {
            std::mem::swap(&mut self, &mut other);
            -R::one()
        } else {
            R::one()
        };
        for (c1, c2) in self.coefficients.iter_mut().zip(other.coefficients) {
            *c1 = a.clone() * c1.clone() + (-R::one() * a.clone()) * c2;
        }
        for i in (other.degree + 1)..(self.degree + 1) {
            self.coefficients[i] *= a.clone();
        }
        return self;
    }
}

impl<'a, R: Ring<'a>> Mul<Self> for UPolynomial<'a, R> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut new_coefs = vec![R::zero(); self.degree + other.degree + 1];
        for k in 0..(self.degree + other.degree + 1) {
            for i in 0..(k + 1) {
                let j = k - i;
                println!("i:{}, j:{}", i, j);
                if i <= self.degree && j <= other.degree {
                    new_coefs[k] += self.coefficients[i].clone() * other.coefficients[j].clone();
                }
            }
        }
        Self {
            coefficients: new_coefs,
            degree: self.degree + other.degree,
            phantom: PhantomData,
        }
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

impl<'a, R: Ring<'a>> Mul<&'a Self> for UPolynomial<'a, R> {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self {
        let other = other.clone();
        self * other
    }
}
/*
impl<'a, R: Ring<'a>> Ring<'a> for UPolynomial<'a, R> {}

impl<'a, R: Ring<'a>> Polynomial<'a> for UPolynomial<'a, R> {
    type CoefRing = R;
    fn dim(&self) -> usize {
        1
    }
}
*/

/// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let p = UPolynomial::new(vec![2, -3, 5]);
        let q = UPolynomial::new(vec![4, 9, 4, 3]);
        let expected = UPolynomial::new(vec![6, 6, 9, 3]);
        assert_eq!(p + q, expected);
    }

    #[test]
    fn test_sub() {
        let p = UPolynomial::new(vec![2, -3, 5]);
        let q = UPolynomial::new(vec![4, 9, 4, 3]);
        let expected = UPolynomial::new(vec![-2, -12, 1, -3]);
        assert_eq!(p - q, expected);
    }

    #[test]
    fn test_mul() {
        let p = UPolynomial::new(vec![2, -3, 5]);
        let q = UPolynomial::new(vec![4, 9, 4, 3]);
        let expected = UPolynomial::new(vec![8, 6, 1, 39, 11, 15]);
        assert_eq!(p * q, expected);
    }
}
