use std::marker::PhantomData;

use crate::ring::Ring;

pub struct Matrix<'a, R: Ring<'a>> {
    coefs: Vec<R>,
    dim: (usize, usize),
    _phantom: PhantomData<&'a R>,
}
