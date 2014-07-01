fn main() {
    let mut num_count = 0u;
    let mut p_count = 0u;
    let largest_palindrome = prod_iter()
        .inspect(|_| num_count += 1)
        .filter(num_is_palindrome)
        .inspect(|_| p_count += 1)
        .fold(0, max);
    println!("largest palindrome: {}", largest_palindrome);
    println!("numbers: {}, palindromes: {}", num_count, p_count);
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
    ProdIter { a: 900, b: 900, max: 999, min: 900 }
}

fn num_is_palindrome(i: &u64) -> bool {
    let string = i.to_str();
    let reverse = reverse_string(&string);
    string == reverse
}

fn reverse_string(string: &String) -> String {
    let mut new_string = String::new();
    for c in string.as_slice().chars().rev() {
        new_string.push_char(c);
    }
    new_string
}

fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}
