use std::iter::count;

fn main() {
    let drange = DivisibleByRange{ min: 2, max: 20 };
    let result = count(1u64, 1)
        .skip_while(|&n| !drange.test(n))
        .take(1)
        .fold(0, |_, x| x);
    println!("{}", result)
}

struct DivisibleByRange { min: u64, max: u64 }

impl DivisibleByRange {
    fn test(&self, num: u64) -> bool {
        for i in range(self.min, self.max + 1) {
            if num % i != 0 { return false }
        }
        true
    }
}
