# Largest Prime Factor

## Problem Statement

<blockquote>
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
</blockquote>

## Initial thoughts

I think the hardest part of this is going to be figuring out the math. Time to look up "prime factors"!

*reads https://en.wikipedia.org/wiki/Prime_factor*

Okay, that didn't help a *whole* lot.

I've decided the first thing I'm going to try to do to start wrapping my head around this problem is write a program that prints out, say, the first 100 prime numbers.


```rust
// Really stupid, naive way to print out the first 100 primes.

fn main() {
    let mut count = 0i;
    let mut i = 2i;
    let mut is_prime;
    loop {
        is_prime = true;
        if count >= 100 {
            break;
        }
        if i < 4 {
            is_prime = true
        }
        else {
            for ii in range(2, i) {
                if i % ii == 0 {
                    is_prime = false
                }
            }
        }

        if is_prime {
            count += 1;
            println!("{} is prime", i);
        }
        i += 1;
    }
}
```

This is gross as fuck, but it seems to generate a correct list, checking my work against [the list of the first 500 prime numbers](https://en.wikipedia.org/wiki/List_of_prime_numbers#The_first_500_prime_numbers).

So given that, I'm gonna create an iterator that generates all of the prime numbers:

```rust
struct PrimeIter { init_val: int }

fn prime_iter() -> PrimeIter {
    PrimeIter { init_val: 1 }
}

fn divisible_by(a: int, b: int) -> bool {
    a % b == 0
}

impl Iterator<int> for PrimeIter {
    fn next(&mut self) -> Option<int> {
        let mut is_prime;
        loop {
            self.init_val += 1;
            let num = self.init_val;
            // the "innocent until proven guilty" method
            is_prime = true;
            if num < 3 {
                return Some(self.init_val)
            }
            for i in range(2, num) {
                if divisible_by(num, i) {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                return Some(num);
            }
        }
    }
}
```
Well, I got that done, and it works. But I still didn't quite understand prime factorization, so I searched "how to do prime factorization" and found this site: http://www.mathsisfun.com/prime-factorization.html.

RAD, okay, this makes sense – I now get what the question is asking for.

After some fucking around, I stumbled my way to a solution:

```rust
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
```