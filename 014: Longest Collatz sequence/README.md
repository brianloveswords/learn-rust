# Largest Collatz sequence

## Problem statement

```
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
```

## Initial thoughts

* Collatz sequence struct (w/ Show, "x → y → z → a → b → c")
  * `.new(start: u64) -> CollatzSeq`
* `range(1, 1000000).rev()`


## First solution

It works but it's a little slow: it takes about 8 seconds on my machine to come up with a result. This isn't terrible, but I want to see how I can make it faster. As I see it there are two ways this can go: threads to parallelize the map, or a dictionary to store previous Collatz sequences so we can lookup to see if a previous chain has been calculated.

```
use std::fmt;
struct CollatzSeq {
    seq: Vec<u64>
}
impl CollatzSeq {
    fn push(&mut self, n: u64) {
        self.seq.push(n);
    }
    fn len(&self) -> uint {
        self.seq.len()
    }
    fn new(start: u64) -> CollatzSeq {
        let mut i = start;
        let mut collatz = CollatzSeq{ seq: Vec::new() };
        collatz.push(i);
        while i > 1 {
            if i % 2 == 0 {
                i = i / 2
            }
            else {
                i = 3*i + 1
            }
            collatz.push(i);
        }
        collatz
    }
}
impl fmt::Show for CollatzSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string: Vec<String> = self.seq.iter().map(|&x| x.to_str()).collect();
        write!(f, "{}", string.connect(" → "))
    }
}

fn main() {
    let result = range(1u64, 1000000)
        .rev()
        .map(CollatzSeq::new)
        .fold(CollatzSeq::new(1), |a, c| if c.len() > a.len() { c } else { a } );
    println!("{}", result);
}
```

## Second solution

The first thing I tried was parallel processing. Everything other than `main()` pretty much stayed the same. This worked and did improve the execution time – down to about 4 seconds from 8 seconds. So using all four cores only got me double the speed increase. Looking at Instruments, it looks like about ~2000ms of CPU time (so ~500ms of wall time?) is spent doing reallocations, so I changed `Vec::new()` to `Vec::with_capacity(200)` in `CollatzSeq::new`. That helped shave off some time, so now I'm hovering around 3 seconds.

```rust
fn main() {
    let max: u64 = 1000000;
    let cores: u64 = 4;
    let (tx, rx): (Sender<CollatzSeq>, Receiver<CollatzSeq>) = channel();

    for idx in range_inclusive(1, cores) {
        let child_tx = tx.clone();
        spawn(proc() {
            let start = if idx == 1 { 1 } else { max/cores * (idx - 1) };
            let end = max/cores * idx;
            println!("start, end: {}, {}", start, end);
            let result = range(start, end)
                .rev()
                .map(CollatzSeq::new)
                .fold(CollatzSeq::new(1), |a, c| if c.len() > a.len() { c } else { a } );
            child_tx.send(result)
        });
    }

    let mut results = Vec::with_capacity(4);
    for _ in range_inclusive(1, cores) {
        results.push(rx.recv());
    }
    println!("{}", results.iter().fold(&CollatzSeq::new(1), |a, c| if c.len() > a.len() { c } else { a } ));
}
```

## Third solution

While I did learn a lot about using HashMaps (and also that it's possible to use _'s as separators in numbers), performance-wise this turned out *hilariously* bad. Average run for the threaded version above is about 2700ms, with a max memory usage of 1mb (if I'm reading Instruments correctly).

The HashMap version takes about *45000ms* and consumes *1.16gb* of memory. So it's just over 16x worse on time and over 1000x worse on space. ROCK SOLID JOB A++++.

```rust
use std::fmt;
use std::collections::HashMap;

#[deriving(Clone)]
struct CollatzSeq {
    seq: Vec<u64>
}

impl CollatzSeq {
    fn len(&self) -> uint {
        self.seq.len()
    }
    fn new<'a>(start: u64, memo: &'a mut HashMap<u64, CollatzSeq>) -> CollatzSeq {
        let mut i = start;
        let mut collatz = CollatzSeq{ seq: Vec::new() };
        collatz.seq.push(i);
        while i > 1 {
            if i % 2 == 0 {
                i = i / 2
            }
            else {
                i = 3*i + 1
            }

            let found = memo.find(&i);
            if found.is_some() {
                let chain = found.unwrap();
                collatz.seq = collatz.seq.append(chain.seq.as_slice());
                break;
            }
            collatz.seq.push(i);
        }
        memo.insert(start, collatz.clone());
        collatz
    }
    fn new_empty() -> CollatzSeq {
        CollatzSeq { seq: Vec::new() }
    }
}

impl fmt::Show for CollatzSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string: Vec<String> = self.seq.iter().map(|&x| x.to_str()).collect();
        write!(f, "[{}]", string.connect(" → "))
    }
}

fn main() {
    let mut stuff: HashMap<u64, CollatzSeq> = HashMap::new();
    let result = range(1u64, 1_000_000)
        .rev()
        .map(|n| CollatzSeq::new(n, &mut stuff))
        .fold(CollatzSeq::new_empty(), |a, c| if c.len() > a.len() { c } else { a } );
    println!("result: {}", result)
}
```

## Addendum

I just learned about the `--opt-level 3` flag for the compiler. This changes the whole game:

Version 1: ~1200ms (baseline)
Version 2: ~200ms (6x better)
Version 3: ~6000ms (5x *worse*)

With compiler optimization the second version is showing even better than the 4x improvement I initially expected! The 3rd version is of course still garbage, but not nearly as bad as it was.
