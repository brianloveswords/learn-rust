# Largest palindrome product

## Problem statement

```
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
```

## Initial thoughts

Oh boy, string/digit processing.

So here's my initial strategy:
* Make iterator for list of all products of 3-digit numbers
* Filter out non-palidrome numbers (int-to-str converstion)
* Find the max

This is probably going to be wildly inefficient.


## First attempt

* Figuring out how to reverse a string is hard. thought I could use `.chars()`, but apparently that only works on static strings?
* Finally figured out about the `.as_slice()` method by [reading the source of std::str](http://doc.rust-lang.org/src/collections/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcollections/str.rs.html#829).
* Was hoping to be able to do something like `string.shift_char`, but that doesn't work :/
* Came up with a silly way to reverse a string.

Here's my first pass at a solution. The hardest part *by far* was coming up with the function for reversing a string. The tutorial doesn't really teach that much string manipulation, and the docs are really lacking in that area. I know there's gotta be a better way to reverse the string, so my next attempt will be to figure that out.


```rust
use std::char;

fn main() {
    let largest_palindrome = prod_iter()
        .filter(num_is_palindrome)
        .fold(0, max);
    println!("largest palindrome: {}", largest_palindrome);
}

struct ProdIter { a: u16, b: u16, max: u16, min: u16 }

impl Iterator<u64> for ProdIter {
    fn next(&mut self) -> Option<u64> {
        if self.b > self.max {
            self.b = self.min;
            self.a = self.a + 1;
        }
        if self.a > self.max {
            return None
        }
        let product = (self.a as u64) * (self.b as u64);
        self.b = self.b + 1;
        Some(product)
    }
}

fn prod_iter() -> ProdIter {
    ProdIter { a: 100, b: 100, max: 999, min: 100 }
}

fn num_is_palindrome(i: &u64) -> bool {
    let string = i.to_str();
    let reverse = reverse_string(&string);
    string == reverse
}

fn reverse_string(string: &String) -> String {
    let mut new_string = String::new();
    let mut i = (*string).len();

    while i != 0 {
        i -= 1;
        let byte = (*string).as_slice()[i];
        let chr = char::from_u32(byte as u32).unwrap();
        new_string.push_char(chr);
    }

    new_string
}
fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}
```

## Second attempt: small improvement to reverse string

The only change here is the `reverse_string` method:

```rust
fn reverse_string(string: &String) -> String {
    let mut new_string = String::new();
    for c in string.as_slice().chars().rev() {
        new_string.push_char(c);
    }
    new_string
}
```

Looking through [the std::iter  docs](http://doc.rust-lang.org/std/iter/) I found out about `.rev()` which let me use `.chars()` in the way I originally wanted. I'm still not sure it's the *best* way to reverse a string, but it's definitely way more readable.

Unsurprisingly, it's still really slow. String reversing wasn't the bottleneck – `prod_iter()` generates a huge space to search for the answer in (810,000 elements), which has to be fully iterated over by the `filter`.

So while I think the method I have is "pure" in the sense that it *does* search the whole space, it's "dumb" in that it *searches the whole space* – we know the answer is going to be somewhere near the end, definitely somewhere above 900 * 900. A simple optimization would be to change `prod_iter` to

```rust
fn prod_iter() -> ProdIter {
    ProdIter { a: 900, b: 900, max: 999, min: 900 }
}
```

which only has to run through 10,000 elements to find the answer out.
