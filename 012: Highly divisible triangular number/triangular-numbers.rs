use std::iter::range_inclusive;

fn main() {
    let mut iter = triangle_numbers()
        .map(|n| NumberWithDivisors{ number: n, divisors: divisors(n) })
        .skip_while(|x| x.divisors.len() <= 500 );

    for n in iter {
        println!("{} -> {}", n.number, n.divisors);
        break
    }
}

struct NumberWithDivisors {
    number: u64,
    divisors: Vec<u64>,
}

fn triangle_numbers() -> TriangleNumberIter {
    TriangleNumberIter { idx: 0, prev: 0 }
}
struct TriangleNumberIter {
    idx: u64,
    prev: u64,
}
impl Iterator<u64> for TriangleNumberIter {
    fn next(&mut self) -> Option<u64> {
        self.idx += 1;
        let result = self.idx + self.prev;
        self.prev = result;
        Some(result)
    }
}

fn divisors(num: u64) -> Vec<u64> {
    let mut result = vec![1];
    let sqrt = (num as f64).sqrt().ceil() as u64;
    for i in range_inclusive(2, sqrt) {
        if num % i == 0 {
            result.push(i);
            result.push(num / i);
        }
    }
    result.push(num);
    result
}
