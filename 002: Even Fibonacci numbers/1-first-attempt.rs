/// For my first attempt, I went at it without really looking up too
/// much. I knew I wanted my `fib` function to be generic, with the
/// logic around figuring out the the sum separate. I also wanted to use
/// a generator, but very quickly figured out that though "yield" is a
/// reserved keyword, it doesn't actually do anything. For some reason,
/// this was the first solution that popped into my head. It's probably
/// strange, and very non-idiomatic, but it worked.

fn fib(pred: |x: int| -> bool) {
    let mut previous = 1;
    let mut current = 1;
    loop {
        if !pred(previous) { break }
        let tmp = current;
        current += previous;
        previous = tmp;
    }
}

fn main() {
    let mut sum = 0;
    fib(|i| -> bool {
        if i > 4000000 {
            // more or less a `break`
            return false
        }

        if i % 2 != 0 {
            // more or less `next`
            return true
        }

        println!("adding {}",  i);
        sum += i;
        true
    });
    println!("sum = {}", sum)
}
