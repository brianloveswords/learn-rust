/// I think I did a lot better with my second try. I thought about
/// vectors, and how they implement `.iter()` and I tried to figure out
/// how that was done. I sort of bumbled around until I discovered the
/// [core::iter::Iterator](http://doc.rust-lang.org/core/iter/trait.Iterator.html),
/// which seemed to be what I was looking for.
///
/// It took me a hot minute to figure out exactly how I was meant to
/// implement it. The explanation for the required method, `next`, says this:
///
///    Advance the iterator and return the next value. Return `None` when
///    the end is reached.
///
/// I got caught up on the "advance the iterator" part. Maybe it might
/// be clearer as "update internal state"?
///
/// Anyway, coming from JavaScript, I was initially resistant to having
/// to declare initial state, so much so that my first attempt at the
/// FibIter struct was just `struct FibIter;` hoping there'd be some way
/// to avoid having to delcare initial state during the construction
/// process, since it will always be the same. It took me a
/// disappointingly long time to realize that I could solve the initial
/// state problem with a constructor function.
///
/// I feel like I still don't know enough about Rust to know if this is
/// the "idiomatic" way to solve the problem, but I feel like it's
/// definitely several steps above my janky first solution.

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
    let mut sum = 0;
    for i in fib_iter() {
        if i > 4000000 {
            break;
        }
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("sum = {}", sum);
}
