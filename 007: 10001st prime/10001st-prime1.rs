struct PrimeIter { state: u64 }

fn main() {
    println!("{}", nth_prime(10001));
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

fn divisible_by(a: u64, b: u64) -> bool {
    a % b == 0
}

fn is_prime(num: u64) -> bool {
    if num < 2 { return false }
    if num < 4 { return true }

    for i in range(2, num) {
        if divisible_by(num, i) {
            return false
        }
    }
    true
}
