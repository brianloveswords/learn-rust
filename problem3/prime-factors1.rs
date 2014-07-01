fn main() {
    let test_number = 600851475143;
    println!("prime factors: {}", prime_factorization(test_number));
}

struct PrimeIter { init_val: i64 }

impl Iterator<i64> for PrimeIter {
    fn next(&mut self) -> Option<i64> {
        loop {
            self.init_val += 1;
            let num = self.init_val;

            if is_prime(num) {
                return Some(num);
            }
        }
    }
}

fn prime_iter() -> PrimeIter {
    PrimeIter { init_val: 1 }
}

fn divisible_by(a: i64, b: i64) -> bool {
    a % b == 0
}

fn is_prime(num: i64) -> bool {
    if num < 2 { return false }
    if num < 4 { return true }

    for i in range(2, num) {
        if divisible_by(num, i) {
            return false
        }
    }
    true
}

fn prime_factorization(num: i64) -> Vec<i64> {
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
                remainder /= prime;

                if is_prime(remainder) {
                    factors.push(remainder);
                    return factors;
                }

            } else { break; }
        }
    }

    factors
}
