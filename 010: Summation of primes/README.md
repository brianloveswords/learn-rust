# Summation of primes

## Problem statement

```
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
```

## Initial thoughts

Given the work I've done on my prime iterator, this should be piece of cake. LET'S DO THIS.


## First attempt

I encapsulated my previous primes iterator work into a module and stuck that in `primes.rs`.

It's a good thing I optimized that prime iterator: this took ~8 seconds with the optimization, I can't imagine how long it would take without.

```
mod primes;

fn main() {
    let sum = primes::iter()
        .take_while(|&x| x < 2000000)
        .fold(0, |a, x| a + x);
    println!("{}", sum)
}
```
