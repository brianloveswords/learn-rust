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
