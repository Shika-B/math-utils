mod matrix;
mod ring;

use ring::Ring;

fn p1<T: Ring>(x: T) -> T {
    x + T::one()
}

fn main() {
    println!("{:?}", p1(2));
}
