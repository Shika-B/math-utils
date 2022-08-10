pub mod arithmetic;
pub mod mod_int;
pub mod polynomial;
pub mod primitive;

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, BitAnd, Mul, MulAssign, Neg, Shl, Sub, SubAssign},
};

use num::{Integer, One, Zero};

pub use num::pow;

pub trait Ring<'a>:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + Zero
    + One
    + Neg<Output = Self>
    + PartialEq
    + Debug
    + Clone
where
    Self: 'a,
{
    /// HACK: There are no type methods, so just use `R::zero().is_field()`
    fn is_field(&self) -> Option<bool> {
        None
    }

    /// HACK: There are no type methods, so just use `R::zero().is_commutative()`
    fn is_commutative(&self) -> bool {
        false
    }

    fn inverse(&self) -> Option<Self> {
        None
    }
}

pub trait Ringed<'a, R>
where
    R: Ring<'a>,
{
    fn ringed(self) -> R;
}

/// Takes an integer `s` and returns the image of the `s` under the unique ring morphism `f: Z -> R`.
fn ringed<'a, R: Ring<'a>, N: Integer + BitAnd<Output = N> + Shl<Output = N> + Clone>(s: N) -> R {
    // sum fast
    let mut result = R::zero();
    let mut mask = N::one();
    let mut bin_sum = R::one();
    for _ in 0..127 {
        if mask > s.clone() {
            break;
        }
        let is_set = (s.clone() & mask.clone()) != N::zero();
        if is_set {
            result += bin_sum.clone();
        }
        mask = mask.clone() << N::one();
        bin_sum += bin_sum.clone();
    }
    result
}
