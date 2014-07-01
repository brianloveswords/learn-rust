use std::char;

fn main() {
    let largest_palindrome = prod_iter()
        .filter(num_is_palindrome)
        .fold(0, max);
    println!("largest palindrome: {}", largest_palindrome);
}

struct ProdIter { a: u16, b: u16, max: u16, min: u16 }

impl Iterator<u64> for ProdIter {
    fn next(&mut self) -> Option<u64> {
        if self.b > self.max {
            self.b = self.min;
            self.a = self.a + 1;
        }
        if self.a > self.max {
            return None
        }
        let product = (self.a as u64) * (self.b as u64);
        self.b = self.b + 1;
        Some(product)
    }
}

fn prod_iter() -> ProdIter {
    ProdIter { a: 100, b: 100, max: 999, min: 100 }
}

fn num_is_palindrome(i: &u64) -> bool {
    let string = i.to_str();
    let reverse = reverse_string(&string);
    string == reverse
}

fn reverse_string(string: &String) -> String {
    let mut new_string = String::new();
    let mut i = (*string).len();

    while i != 0 {
        i -= 1;
        let byte = (*string).as_slice()[i];
        let chr = char::from_u32(byte as u32).unwrap();
        new_string.push_char(chr);
    }

    new_string
}
fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}
