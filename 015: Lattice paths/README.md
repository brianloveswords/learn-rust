# Lattice paths

## Problem statement

<pre><code>
Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

![paths](https://projecteuler.net/project/images/p_015.gif)

How many such routes are there through a 20×20 grid?
</code></pre>

## Initial thoughts

Thinking about it a little bit, in a 2x2 grid, with an actor that can only move down and right, there will always be exactly 4 moves to get there. Here's a textual representation of the movesets:

```
[R,R,D,D]
[R,D,R,D]
[R,D,D,R]
[D,R,R,D]
[D,R,D,R]
[D,D,R,R]
```

There are also always an equal number of `R`s and `D`s.

Let me think about a 3x3 grid:

```
[R,R,R,D,D,D]
[R,R,D,R,D,D]
[R,R,D,D,R,D]
[R,R,D,D,D,R]
[R,D,R,R,D,D]
[R,D,D,R,R,D]
[R,D,D,R,D,R]
[R,D,D,D,R,R]
[... swap D and R, repeat ...]
```

So by my calculations there are 16 paths for a 3x3 grid. (NOTE: my calculations were wrong).

## First solution

I tried really hard to figure out the solution without looking up an algorithm for it to see if I could find it experimentally. I initially tried doing something with `.permutations()` and `HashSet` which worked for small grids, but started breaking down at around grids around size 5 and at size 7 it seemed like it was never going to finish.

For a little while I started trying to figure out if I could discern a relationship between the series I was able to come up with (6, 20, 70, 252, 924) but I couldn't come up with any simple mathematical relationship.

After that I started looking up "lattice paths" and from there learned about the [binomial coefficient](http://mathworld.wolfram.com/BinomialCoefficient.html) which is the exact equation I was looking for. That particular site was overwhelming, so I found [another site](http://www.robertdickau.com/manhattan.html) that explained it in language that I found easier to understand (or at least translate to code).

Once I knew the correct formula, implementation was fairly trivial. The one thing that tripped me up at first was neglecting to use BigUint and running into overflow errors.

```rust
extern crate num;
use std::num::One;
use num::bigint::BigUint;
use std::num::FromPrimitive;
use std::iter::range_inclusive;

fn factorial(n: u64) -> BigUint {
    let one = One::one();
    let mut product: BigUint = one;
    for i in range_inclusive(2, n) {
        product = product * FromPrimitive::from_u64(i).unwrap();
    }
    product
}

fn square(n: BigUint) -> BigUint {
    n * n
}

fn binomial_coefficient(n: u64) -> BigUint {
    factorial(2*n)/square(factorial(n))
}

fn main() {
    println!("{}", binomial_coefficient(6))
}
```