# Smallest multiple

## Problem Statement
```
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
```

## Initial thoughts

* Natural numbers iterator
* `divisble_up_to(max: u8, value: u64)`
* `.skip_while()`

## First attempt (success)

Pretty close to my original thoughts, used a struct for doing the divisible test instead of a function.

I don't *really* like having to do that `.fold(0, |_, x| x|)` to pull off the last (only) value. I wonder if there's a better way to do that?

Also, on my computer this is quite slow: `time ./smallest-multiple` returns about 24 seconds of 100% CPU.

```rust
fn main() {
    let drange = DivisibleByRange{ min: 2, max: 20 };
    let result = numbers_iter()
        .skip_while(|&n| !drange.test(n))
        .take(1)
        .fold(0, |_, x| x);
    println!("{}", result)
}

struct NaturalNumIter { value: u64 }

struct DivisibleByRange {
    min: u64,
    max: u64,
}

impl DivisibleByRange {
    fn test(&self, num: u64) -> bool {
        for i in range(self.min, self.max + 1) {
            if num % i != 0 { return false }
        }
        true
    }
}

impl Iterator<u64> for NaturalNumIter {
    fn next(&mut self) -> Option<u64> {
        let orig = self.value;
        self.value = self.value + 1;
        Some(orig)
    }
}
fn numbers_iter() -> NaturalNumIter {
    NaturalNumIter { value: 1 }
}
```

## Second attempt: minor improvement

Looking at [std::iter](http://doc.rust-lang.org/std/iter/) I noticed `Counter` which seemed like exactly the struct I was looking for to create an infinite iterator of natural numbers. [Reading through the source](http://doc.rust-lang.org/src/core/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcore/iter.rs.html#1911-1916), I was able to find a public function `count()` that creates an instance of that iterator, so I replaced my `NaturalNumIter` with that, simplifying the code a bit.

This doesn't do anything to address the performance issue.

```rust
use std::iter::count;

fn main() {
    let drange = DivisibleByRange{ min: 2, max: 20 };
    let result = count(1u64, 1)
        .skip_while(|&n| !drange.test(n))
        .take(1)
        .fold(0, |_, x| x);
    println!("{}", result)
}

struct DivisibleByRange { min: u64, max: u64 }

impl DivisibleByRange {
    fn test(&self, num: u64) -> bool {
        for i in range(self.min, self.max + 1) {
            if num % i != 0 { return false }
        }
        true
    }
}
```