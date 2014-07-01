#![feature(macro_rules)]
use std::num::FromPrimitive;

macro_rules! from_int {
    ($expression:expr) => {
        FromPrimitive::from_int($expression).unwrap()
    }
}

fn main() {
    let big_number = 600851475143u64;
    let small_number = 398u16;

    println!("big number: {}", prime_factorization(big_number));
    println!("small number: {}", prime_factorization(small_number));
}

struct PrimeIter<T> { init_val: T }

impl<T: Int + FromPrimitive> Iterator<T> for PrimeIter<T> {
    fn next(&mut self) -> Option<T> {
        loop {
            self.init_val = self.init_val + from_int!(1);
            let num = self.init_val;

            if is_prime(num) {
                return Some(num);
            }
        }
    }
}

fn prime_iter<T: FromPrimitive>() -> PrimeIter<T> {
    PrimeIter { init_val: from_int!(1) }
}

fn divisible_by<T: Int + FromPrimitive>(a: T, b: T) -> bool {
    a % b == from_int!(0)
}

fn is_prime<T: Int + FromPrimitive>(num: T) -> bool {
    if num < from_int!(2) {
        return false
    }
    if num < from_int!(4) {
        return true
    }

    let begin = from_int!(2i);
    for i in range(begin, num) {
        if divisible_by(num, i) {
            return false
        }
    }
    true
}

fn prime_factorization<T: Int + FromPrimitive>(num: T) -> Vec<T> {
    let mut remainder = num.clone();
    let mut factors = Vec::new();

    if is_prime(num) {
        factors.push(num);
        return factors
    }

    for prime in prime_iter() {
        loop {
            if divisible_by(remainder, prime) {
                factors.push(prime);
                remainder = remainder / prime;

                if is_prime(remainder) {
                    factors.push(remainder);
                    return factors;
                }

            } else { break; }
        }
    }

    factors
}
