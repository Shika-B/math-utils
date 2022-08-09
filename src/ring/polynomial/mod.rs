use crate::ring::Ring;

// Todo write a macro_rule to create a polynomial in a nice way. Example syntax:
// `let p = polynomial!(3x^2 + 2x + 5);`
pub mod univar_polynomial;

//TODO Write a generic multivariate polynomial using macros
pub mod multivar_polynomial;

pub trait Polynomial: Ring {
    type CoefRing: Ring;
    fn dim(&self) -> usize;
}
