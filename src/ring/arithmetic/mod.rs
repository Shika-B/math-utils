use num::Integer;

pub use num::pow;

// Basically just reimplement `num::Integer`'s method that I judged useful
pub fn xgcd<N: Integer + Clone>(n1: &N, n2: &N) -> (N, N, N) {
    let a = n1.extended_gcd(n2);
    (a.gcd, a.x, a.y)
}

pub fn gcd<N: Integer>(n1: &N, n2: &N) -> N {
    n1.gcd(n2)
}

pub fn lcm<N: Integer>(n1: &N, n2: &N) -> N {
    n1.lcm(n2)
}
