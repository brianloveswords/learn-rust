extern crate num;
use std::num::One;
use num::bigint::BigUint;
use std::num::FromPrimitive;
use std::iter::range_inclusive;

fn factorial(n: u64) -> BigUint {
    let one = One::one();
    let mut product: BigUint = one;
    for i in range_inclusive(2, n) {
        product = product * FromPrimitive::from_u64(i).unwrap();
    }
    product
}

fn square(n: BigUint) -> BigUint {
    n * n
}

fn binomial_coefficient(n: u64) -> BigUint {
    factorial(2*n)/square(factorial(n))
}

fn main() {
    println!("{}", binomial_coefficient(6))
}
