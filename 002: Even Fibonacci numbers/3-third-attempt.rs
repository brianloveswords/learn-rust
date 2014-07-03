/// Ahh, now here's the ticket. I dug a little deeper and realized that
/// you get things like `take_while` and `filter` for just by
/// implementing the Iterator trait. With that I got to use my favorite
/// form of problem solving: starting with an infinite set and reducing
/// down to the correct answer.
///
/// I'd be curious to see the performance difference between this
/// solution and my previous attempt that uses an explicit `for`
/// loop but I have no idea how to benchmark at the moment.
///
/// I really like this method of solving the problem though. I'm sadly
/// used to not being able to use this technique as much in JavaScript
/// when performance is a concern since each call to methods like
/// `filter`, `reduce` and `map` incurs a full array traversal (note: I
/// don't know if we're going to get these methods on iterators once
/// they land in ES6).

struct FibIter {
    previous: int,
    current: int
}

fn fib_iter() -> FibIter {
    FibIter { previous: 1, current: 1 }
}

impl Iterator<int> for FibIter {
    fn next(&mut self) -> Option<int> {
        let tmp = self.previous;
        self.previous = self.current;
        self.current += tmp;
        Some(tmp)
    }
}

fn main() {
    let sum = fib_iter()
        .take_while(|&i| i < 4000000)
        .filter(|&i| i % 2 == 0)
        .fold(0, |acc, i| acc + i);
    println!("sum = {}", sum);
}