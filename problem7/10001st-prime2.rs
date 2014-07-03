use std::iter::range_step_inclusive;

struct PrimeIter { state: u64 }

fn main() {
    println!("{}", nth_prime(100001));
}

impl Iterator<u64> for PrimeIter {
    fn next(&mut self) -> Option<u64> {
        loop {
            self.state += 1;
            let num = self.state;

            if is_prime(num) {
                return Some(num);
            }
        }
    }
}

fn nth_prime(n: uint) -> u64 {
    prime_iter()
        .skip(n - 1)
        .take(1)
        .fold(0, |_, x| x)
}

fn prime_iter() -> PrimeIter {
    PrimeIter { state: 1 }
}

fn is_prime(num: u64) -> bool {
    if num == 2 {
        return true
    }

    if num < 2 || num % 2 == 0 {
        return false
    }

    let limit = sqrt(num);

    for i in range_step_inclusive(3, limit, 1) {
        if num % i == 0 { return false }
    }
    true
}

fn sqrt(num: u64) -> u64 {
    ((num as f64).sqrt().ceil()) as u64
}
