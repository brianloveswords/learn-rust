# Largest Prime Factor

## Problem Statement

```
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
```

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

## First Attempt: getting a solution

I imagine this is really inefficient (but then again, isn't that the whole point of prime factorization?): `is_prime` iterates over a range every time it's called and it's called within a loop.

It's also a little scary to me that only condition that ends the `for prime in prime_iter()` iteration is deeply nested in conditionals, but I can be confident that will always be hit, either by the promise of the compiler(?) or the fundamental theorem of arithmetic.

I originally coded this up using `int` instead of `i64` but when I put in `600851475143` as my test number, the compiler rightfully warned that it would overflow, which is *awesome*. I changed everything to use `i64` instead, but that's probably a bit wasteful (and I should probably use `u64`)

An even better technique might be to use generic types, assuming I understand them correctly. This would make the code more reusable while saving some space in situations where input value fits into a smaller integer type.

**Open question**: I wonder how using a generic type would affect a case where the input is coming from, say, the command line and couldn't be statically analyzed ahead of time? Would the compiled code be any different than in the case where the input can be statically analyzed?


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

## Second Attempt: making it generic

* Oh wow much harder than I thought.
* Error messages were confusing for me.
* After searching for "rust non-scalar cast", [learned about `FromPrimitive`](https://stackoverflow.com/questions/21073709/compare-a-generic-number-argument-with-a-constant)
* Learned about having to `.unwrap()` [from reading source]( http://doc.rust-lang.org/src/num/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libnum/lib.rs.html#11-69)

Again, I stumbled my way to a solution. It was really hard to figure out all the necessary incantations (traits) for each of the functions to get to work. I also spent a long time struggling against `+=` because it seems like the `Add` trait doesn't cover that! I kept getting a "binary assignment operation `+=` cannot be assigned to type `T`" error, but if I expanded it to a regular assignment, it worked fine.

I was only able to eventually figure it out by searching for "rust trait toprimitive" and somehow finding [this page which lists all the traits inherited by type `uint`](http://doc.rust-lang.org/core/uint/primitive.uint.html). I have no idea where that's linked from in the actual docs, though!

In the end I'm not sure the source complexity is actually worth it in this case – might as well just use `u64` types throughout and call it a day.

```rust
use std::num::FromPrimitive;

fn main() {
    let big_number = 600851475143u64;
    let small_number = 398u16;

    println!("big number: {}", prime_factorization(big_number));
    println!("small number: {}", prime_factorization(small_number));
}

struct PrimeIter<T> { init_val: T }

impl<T: Add<T, T> + Int + Primitive + Ord + FromPrimitive> Iterator<T> for PrimeIter<T> {
    fn next(&mut self) -> Option<T> {
        let one = FromPrimitive::from_int(1).unwrap();
        loop {
            self.init_val = self.init_val + one;
            let num = self.init_val;

            if is_prime(num) {
                return Some(num);
            }
        }
    }
}

fn prime_iter<T: FromPrimitive>() -> PrimeIter<T> {
    PrimeIter { init_val: FromPrimitive::from_int(1).unwrap() }
}

fn divisible_by<T: Int + FromPrimitive>(a: T, b: T) -> bool {
    a % b == FromPrimitive::from_int(0).unwrap()
}

fn is_prime<T: Primitive + Int + Ord + FromPrimitive>(num: T) -> bool {
    if num < FromPrimitive::from_int(2).unwrap() {
        return false
    }
    if num < FromPrimitive::from_int(4).unwrap() {
        return true
    }

    let begin = FromPrimitive::from_int(2i).unwrap();
    for i in range(begin, num) {
        if divisible_by(num, i) {
            return false
        }
    }
    true
}

fn prime_factorization<T: Int + Ord + FromPrimitive>(num: T) -> Vec<T> {
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
```

## Third Attempt: macros! and trait cleanup

I wanted to do one last attempt, mostly to figure out how to write macros and abstract out those `FromPrimitive::from_int(...).unwrap()` calls that had to happen all over the place.

I also wanted to see if I could cleanup the traits a little bit, since I got to it above mostly by blind luck (and listening to the compiler yell at me about what was missing). I was successful in getting rid of most traits except for `Int` and `FromPrimitive`.

With the removal of the extraneous traits and the use of the `from_int!` macro, this actually doesn't look so bad. It might be cool if the compiler could somehow warn of unnecessary traits (e.g., `<T: Int + PartialEq>`, `PartialEq` would be unnecessary since it's inherited through the chain `Int -> Primitive -> PartialOrd -> PartialEq`, as far as I understand)

```rust
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
```