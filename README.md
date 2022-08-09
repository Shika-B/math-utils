TODO
- [x] Basic `Ring` and `Polynomial` traits
- [ ] Basic Rings:
    - [x] Given a ring `R`, univariate polynomials over `R`, that is `R[X]`
    - [ ] Given a ring `R`, `n`-variate polynomials, that is `R[X_1, ..., X_n]`
    - [x] `R` (`f32`, `f64`)
    - [x] `Q` (using `BigRational`, `Rational32` and `Rational64` from the `num_rational` crate)
    - [x] `Z` (using `i8, i16, i32, i64, i128` from the `std` and `BigInt` from the `num_bigint` crate)
    - [ ] Integers mod N using `num_bigint`
- [ ] Basic modular arithmetic utilities:
    - [ ] Inverses
    - [ ] Square roots and more generally `n`-th root, in particular `n`-th roots of unity
    - [ ] Modular exponentiation (interface for `pow_mod` from `num_bigint`)
- [ ] Add optional `is_field`, `is_commutative` etc. methods to the `Ring` trait.
- [ ] Add a product of rings structure, typically to be able to work over `Z^n` or `Q^n`.
- [ ] Modules (vector spaces over rings) traits and implementations.
- [ ] Implement fast polynomial multiplication using Karatsuba's algorithm atleast. 



