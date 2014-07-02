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
