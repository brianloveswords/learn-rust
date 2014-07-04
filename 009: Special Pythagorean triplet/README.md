# Special Pythagorean triplet

## Problem statement

A Pythagorean triplet is a set of three natural numbers, *a < b < c*, for which,

  *a² + b² = c²*

For example, 3² + 4² = 9 + 16 = 25 = 5².

There exists exactly one Pythagorean triplet for which *a + b + c* = 1000.
Find the product *abc*.

## Initial thoughts

Create an iterator for the series of Pythagorean triples and stop at the first one that satisfies the condition `a + b + c === 1000`, then take the product.

## First attempt

Nothing too tricky here, I feel. I split `Triplet` and `TripletIter` into separate types even though they contain the same elements.

As usual I want there to be a better way to get just the first valid result of an iteration without having to do either `.collect().get(0)` or `fold(...)`.

```rust
use std::fmt;

fn main() {
    let result: Vec<Triplet> = triplet_iter()
        .skip_while(|x| x.a + x.b + x.c != 1000)
        .take(1)
        .collect();
    println!("{}", result.get(0).product());
}

struct Triplet { a: u64, b: u64, c: u64 }
struct TripletIter { a: u64, b: u64, c: u64 }

impl Triplet {
    fn product(&self) -> u64 {
        self.a * self.b * self.c
    }
}

impl PartialEq for Triplet {
    fn eq(&self, other: &Triplet) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl fmt::Show for Triplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[a: {}, b: {}, c: {}]", self.a, self.b, self.c)
    }
}

impl Iterator<Triplet> for TripletIter {
    fn next(&mut self) -> Option<Triplet> {
        loop {
            self.a = self.a + 1;
            if self.a < self.b {
                let sum = (square(self.a) + square(self.b)) as f64;
                if sum.sqrt() == sum.sqrt().round() {
                    self.c = sum.sqrt() as u64;
                    return Some(Triplet{
                        a: self.a,
                        b: self.b,
                        c: self.c
                    });
                }
            } else {
                self.a = 1;
                self.b = self.b + 1;
            }
        }
    }
}

fn square(x: u64) -> u64 { x * x }

fn triplet_iter() -> TripletIter {
    TripletIter{ a: 1, b: 1, c: 0 }
}

#[cfg(test)]
mod test {
    #[test]
    fn triplet_iter_test() {
        let x = ::triplet_iter().next();
        assert_eq!(x, Some(::Triplet{a: 3u64, b: 4, c: 5}));
    }
}
```