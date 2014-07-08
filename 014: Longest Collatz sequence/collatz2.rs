use std::fmt;
use std::iter::range_inclusive;
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
        let mut collatz = CollatzSeq{ seq: Vec::with_capacity(200) };
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
    let max: u64 = 1000000;
    let cores: u64 = 4;
    let (tx, rx): (Sender<CollatzSeq>, Receiver<CollatzSeq>) = channel();

    for idx in range_inclusive(1, cores) {
        let child_tx = tx.clone();
        spawn(proc() {
            let start = if idx == 1 { 1 } else { max/cores * (idx - 1) };
            let end = max/cores * idx;
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
