mod primes;

fn main() {
    let sum = primes::iter()
        .take_while(|&x| x < 2000000)
        .fold(0, |a, x| a + x);
    println!("{}", sum)
}
