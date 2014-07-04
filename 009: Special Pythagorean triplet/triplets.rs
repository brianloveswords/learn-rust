use std::fmt;

fn main() {
    let result: Vec<Triplet> = triplet_iter()
        .skip_while(|x| x.sum() != 1000)
        .take(1)
        .collect();
    println!("{}", result.get(0).product());
}

struct Triplet { a: u64, b: u64, c: u64 }
struct TripletIter { a: u64, b: u64, c: u64 }

impl Triplet {
    fn product(&self) -> u64 {
        self.a * self.b * self.c
    }
    fn sum(&self) -> u64 {
        self.a + self.b + self.c
    }
}

impl PartialEq for Triplet {
    fn eq(&self, other: &Triplet) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl fmt::Show for Triplet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[a: {}, b: {}, c: {}]", self.a, self.b, self.c)
    }
}

impl Iterator<Triplet> for TripletIter {
    fn next(&mut self) -> Option<Triplet> {
        loop {
            self.a = self.a + 1;
            if self.a < self.b {
                let sum = (square(self.a) + square(self.b)) as f64;
                if sum.sqrt() == sum.sqrt().round() {
                    self.c = sum.sqrt() as u64;
                    return Some(Triplet{
                        a: self.a,
                        b: self.b,
                        c: self.c
                    });
                }
            } else {
                self.a = 1;
                self.b = self.b + 1;
            }
        }
    }
}

fn square(x: u64) -> u64 { x * x }

fn triplet_iter() -> TripletIter {
    TripletIter{ a: 1, b: 1, c: 0 }
}

#[cfg(test)]
mod test {
    #[test]
    fn triplet_iter_test() {
        let x = ::triplet_iter().next();
        assert_eq!(x, Some(::Triplet{a: 3u64, b: 4, c: 5}));
    }
}
