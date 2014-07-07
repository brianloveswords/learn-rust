extern crate num;
use std::io;
use num::bigint::BigUint;
use std::from_str::FromStr;

fn main() {
    let mut sum = BigUint::new(vec![0]);
    for line in io::stdin().lines() {
        let num: BigUint = FromStr::from_str(line.ok().unwrap().as_slice().trim()).unwrap();
        sum = sum + num;
    }

    let digits: String = sum
        .to_str()
        .as_slice()
        .chars()
        .take(10)
        .collect();

    println!("sum: {}", sum);
    println!("first 10 digits: {}", digits)
}
