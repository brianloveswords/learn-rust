# Power digit sum

## Problem statement

```
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
```

## Initial thoughts

`BigUint` again. I wonder if it has `pow()` or something as a function or if I need to create a loop to do the powering. From there: turn num into a string, loop over the chars, turn each char back into a number, and add 'em up.


## First solution

Pretty much how I thought it would be. I wish there were some better way to initialize `BigUint` with a number – something better than `FromPrimitive` or doing that `vec!` hack (not sure it's really a hack. it just feels that way).

```rust
extern crate num;
use num::bigint::BigUint;
use std::num::pow;

fn main() {
    let num = pow(BigUint::new(vec![2]), 1000);

    let sum = num
        .to_str()
        .as_slice()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |a, x| a + x);

    println!("sum: {}", sum);
}
```