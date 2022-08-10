TODO:

- [x] Basic `Ring` and `Polynomial` traits
- [ ] Basic Rings:
    - [x] Given a ring `R`, univariate polynomials over `R`, that is `R[X]`
    - [ ] Given a ring `R`, `n`-variate polynomials, that is `R[X_1, ..., X_n]`
    - [x] `R` (using `f32`, `f64`)
    - [x] `Q` (using `BigRational`, `Rational32` and `Rational64` from the `num_rational` crate)
    - [x] `Z` (using `i8, i16, i32, i64, i128` from the `std` and `BigInt` from the `num_bigint` crate)
    - [x] Integers mod `n` (using `i128`)
    - [ ] Big integer mod `n` (using `num_bigint`)

- [x] Add an `fn inverse(&self) -> Option<R>` method to the `Ring` trait defaulting to `None`.
- [x] Add optionals `is_field`, `is_commutative`, etc. methods to the `Ring` trait.
- [ ] Add a product of rings structure.

- [ ] Basic modular arithmetic utilities:
    - [ ] Square roots mod prime (Tonelli-Shanks algorithm)
    - [ ] Square roots mod prime powers (Tonelli-Shanks algorithm, see Dickson's book)
    - [ ] Square roots mod any composite integers (??)
    - [ ] More generally`n`-th root, in particular `n`-th roots of unity
    - [x] Modular exponentiation

- [ ] Implement fast polynomial multiplication using Karatsuba's algorithm atleast. 

- [ ] Matrix's `Ring` structure:
    - [ ] Sum
    - [ ] Product
    - [ ] Inverse 

- [ ] Matrix specific operations:
    - [ ] Indexing
    - [ ] Transpose
    - [ ] arrange rows/columns  

- [ ] View a matrix as a system and solve it

- [ ] Modules (vector spaces over rings) traits and implementations.

