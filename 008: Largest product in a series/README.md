# Largest product in a series

## Problem statement

```
The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
```

## Initial thoughts

Oh no, dealing with strings! I still don't have a really strong grasp of messing with strings in Rust.

In any case, here's my first idea:
1. turn that string into a vector of digits.
2. iterate over all the digits
  * take the next 3 digits, multiply them (break if there aren't 3 more digits
  * if it's larger than the current known maximum, store it


## First attempt

I got it working, but I feel like I could probably improve things. `safe_get` took me a while to write because of the lifetimes stuff, which I still don't quite understand. I only now realize I should have just looked at the type signature for `collections::vec::Vec::get`:
`fn get<'a>(&'a self, index: uint) -> &'a T`. Mine the same, only with using a generic type and an Option.

I wonder why `safe_get` isn't part of the standard library – It seems like something that would be generically useful!

A bug I keep running into (and which keeps taking me way too long to realize) is overflowing. I originally had `product_of_next` returning a `uint`. This worked when checking my answer against 4 adjacent digits, as described in the test case of the problem statement, but would not return the correct result when trying 13 adjacent digits.

It took me way too long to realize this was because the result of the multiplication was overflowing the container and wrapping back around. I remember seeing something about `Checked` versions of arithmetic operations when I was exploring number traits, I should look into how to do those.

Another thing I'd like to explore is how to use slices of the original vector instead of using a `for` loop `safe_get` to build a new vector . I don't think this will actually save resources, but I think it *might* make some cleaner code?

The last thing I'd like to improve is reporting: I want to know which *n* digits yielded the largest product. This will likely involve changing `product_of_next` so it returns a `Option<some struct>` instead of the product directly.

```rust
fn main() {
    let number = "..."; // omitted for sake of brevity
    let digits: Vec<uint> = number.chars().map(|c| {
        c.to_digit(10).unwrap()
    }).collect();

    let mut max = 0;
    for i in range(0, digits.len()) {
        match product_of_next(&digits, i, 13) {
            None => break,
            Some(product) => if product > max {
                println!("replacing {} with {}", max, product);
                max = product
            }
        }
    }
    println!("{}", max)
}

fn product_of_next(vec: &Vec<uint>, start: uint, amount: uint) -> Option<u64> {
    let limit = start + amount;
    let mut digits = Vec::new();

    for i in range(start, limit) {
        let maybe_digit = safe_get(vec, i);

        if maybe_digit.is_none() {
            return None
        }

        let digit = *maybe_digit.unwrap();

        digits.push(digit);
    }

    let product = digits.iter().fold(1, |a, &x| a * (x as u64));
    Some(product)
}

fn safe_get<'a, T>(vec: &'a Vec<T>, idx: uint) -> Option<&'a T> {
    if idx >= vec.len() {
        return None
    }
    Some(vec.get(idx))
}
```