# 10001st prime

## Problem statement
```
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
```

## Initial thoughts

I more or less already solved this! I built a prime iterator in a previous problem. Just skip the first 10000, take 1.

## First attempt

And indeed, this was mostly copy paste code:

```rust
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
```

It's slow as fuck though – `time ./10001st-prime` shows that it's taking ~22s to get back an answer. I'm not surprised – my `is_prime` function is horribly naïve.

I actually did some googling on to see how other people are profiling Rust applications and came across [a post by Niko Matsakis](https://mail.mozilla.org/pipermail/rust-dev/2012-April/001558.html) saying that they use Instruments on Mac for profiling the compiler. And hey, if that's good enough for them, that's gotta be good enough for me.

I've never used Instruments before now; it's super rad. After compiling with the `-g` flag, I instrumented my executable and confirmed my suspicion:

![shitty perf](https://i.cloudup.com/SB9YfQSeL0.png)

The `is_prime` function is taking up 19 out of those ~22s. I'm gonna look up some other algorithms for determining whether a number is prime and go about implementing & instrumenting some of those.


## Second attempt

![a lot better](https://i.cloudup.com/BhMcbKDBX0.png)

From ~22000ms to ~200ms, over 100x improvement. I followed one of the algorithms from [this page on "Efficient Prime Number Generation"](https://en.wikibooks.org/wiki/Efficient_Prime_Number_Generating_Algorithms). The code changes from the naive method are minor: we test only odd numbers, and we only test up to the square root of a number.

### Rust related challenges

The first challenge I faced was figuring out how to take the square root of a number. I remembered that it was definitely in the main tutorial, but I wanted to see if I could figure it out from the docs alone.

This proved fruitless. I started from the `num` page hoping it would lest methods or functions related to numbers. Nothing there, so I went to `core` and tried clicking on some of the number types. I figured if nothing else it would lead me to some traits that would have implementations of `sqrt`, but nothing there either.

After struggling with the information architecture for way longer than reasonable, it occured to me to use the actual search functionality :expressionless:. Searching for `sqrt` led me to figure out that it's a method on floats.

The next thing I stumbled over was as god damn off-by-one error: When I moved to using the `sqrt` method, I had to start using `range_step_inclusive` instead of `range_step` and I missed that for a while, giving me weird results.

```rust
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
```