extern crate num;
use num::bigint::BigUint;
use std::num::pow;

fn main() {
    let num = pow(BigUint::new(vec![2]), 1000);

    let sum = num
        .to_str()
        .as_slice()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |a, x| a + x);

    println!("sum: {}", sum);
}
