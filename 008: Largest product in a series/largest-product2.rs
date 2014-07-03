use consts::INPUT;
use std::fmt;
mod consts;

struct Result {
    digits: Vec<uint>,
    product: u64,
}

impl PartialOrd for Result {
    fn partial_cmp(&self, other: &Result) -> Option<Ordering> {
        if self.product > other.product { Some(Greater) }
        else if self.product < other.product { Some(Less) }
        else { Some(Equal) }
    }
}

impl PartialEq for Result {
    fn eq(&self, other: &Result) -> bool {
        self.product == other.product
    }
}

impl fmt::Show for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let digits: Vec<String> = self.digits.iter().map(|x| x.to_str()).collect();
        write!(f, "{} = {}", digits.connect(" × "), self.product)
    }
}

fn main() {
    let digits: Vec<uint> = INPUT.chars().map(|c| {
        c.to_digit(10).unwrap()
    }).collect();

    let mut max = Result { product: 0, digits: vec!(0) };
    for i in range(0, digits.len()) {
        match product_of_next(&digits, i, 13) {
            None => break,
            Some(result) => if result > max {
                println!("{}", max);
                max = result
            }
        }
    }
    println!("{}", max)
}

fn product_of_next(vec: &Vec<uint>, start: uint, amount: uint) -> Option<Result> {
    let limit = start + amount;
    let mut digits = Vec::new();

    for i in range(start, limit) {
        let maybe_digit = safe_get(vec, i);

        if maybe_digit.is_none() {
            return None
        }

        let digit = *maybe_digit.unwrap();

        digits.push(digit);
    }

    let product = digits.iter().fold(1, |a, &x| a * (x as u64));
    Some(Result { product: product, digits: digits })
}

fn safe_get<'a, T>(vec: &'a Vec<T>, idx: uint) -> Option<&'a T> {
    if idx >= vec.len() {
        return None
    }
    Some(vec.get(idx))
}
