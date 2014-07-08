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
