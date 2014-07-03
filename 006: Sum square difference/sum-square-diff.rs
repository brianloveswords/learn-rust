use std::iter::count;

fn main() {
    let amount = 100;

    let sum_of_squares = count(1u, 1)
        .take(amount)
        .map(square)
        .fold(0, sum_accum);

    let sums = count(1u, 1)
        .take(amount)
        .fold(0, sum_accum);

    let square_of_sums = sums * sums;

    println!("sum of squares: {}", sum_of_squares);
    println!("square of sums: {}", square_of_sums);
    println!("difference: {}", square_of_sums - sum_of_squares);
}
fn square(n: uint) -> uint { n * n }
fn sum_accum(a: uint, x: uint) -> uint { a + x }
