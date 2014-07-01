/// I felt like this was pretty straightforward – use a range, throw
/// together some `if` statements and bada-bing-bada-boom, got yourself
/// some sums.

fn main() {
    // range is non-inclusive – [0,1000)
    let mut sum = 0;
    for i in range(0i, 1000) {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    println!("sum = {}", sum)
}
