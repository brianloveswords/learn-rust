fn main() {
    let drange = DivisibleByRange{ min: 2, max: 20 };
    let result = numbers_iter()
        .skip_while(|&n| !drange.test(n))
        .take(1)
        .fold(0, |_, x| x);
    println!("{}", result)
}

struct NaturalNumIter { value: u64 }

struct DivisibleByRange {
    min: u64,
    max: u64,
}

impl DivisibleByRange {
    fn test(&self, num: u64) -> bool {
        for i in range(self.min, self.max + 1) {
            if num % i != 0 { return false }
        }
        true
    }
}

impl Iterator<u64> for NaturalNumIter {
    fn next(&mut self) -> Option<u64> {
        let orig = self.value;
        self.value = self.value + 1;
        Some(orig)
    }
}
fn numbers_iter() -> NaturalNumIter {
    NaturalNumIter { value: 1 }
}
