# Largest product in a grid

## Problem statement

```
In the 20×20 grid below, four numbers along a diagonal line have been marked in red (ed. note: obviously the red is not visible here)

      08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
      49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
      81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
      52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
      22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
      24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
      32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
      67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
      24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
      21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
      78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
      16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
      86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
      19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
      04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
      88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
      04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
      20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
      20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
      01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48

The product of these numbers is 26 × 63 × 78 × 14 = 1788696.

What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
```

## Initial thoughts

God damn, they aren't fucking around now. This is similar to the "digits in a row" problem from before, where I dealt with strings, but kicked up a notch.

So I'm thinking a 2-dimensional vector (a vector containing 20 vectors, each with 20 elements) and four iterators: horizontally, vertically, north-east diagonal, and south-east diagonal. These iterators should return groups of 4 adjacent digits.

The trickiest part will probably be the diagonal iterator.

## First attempt

It works, but what a fucking mess. Instead of building four different iterators, I decided to work with transformations of the digit grid instead. This made the iteration easier since it would always be going in the same direction and I wouldn't have to worry about grouping boundaries (how to tell when the iterator hits an edge).

There are a lot of ways this can be DRYed up, especially the `main()` function. I'd also like to use `&'static [u8]` in a lot of places where I currently use `Vec<u8>` since in most of the cases I don't need these collections to be growable. Doing that would also allow me to use `get()` directly without having to call `as_slice()` since the `&'static [T]` version of `get()` returns an `Option` when seeking out of bounds instead of crashing the process like the `Vec<T>` version does. I'll probably come back to this later because I'm kinda sick of working on this problem for now.


```rust
fn main() {
    let digits = DigitGrid {
        grid: vec![
            vec![08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
            vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
            vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
            vec![52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
            vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
            vec![24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
            vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
            vec![67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
            vec![24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
            vec![21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
            vec![78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
            vec![16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
            vec![86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
            vec![19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
            vec![04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
            vec![88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
            vec![04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
            vec![20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
            vec![20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
            vec![01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48]]
    };

    let mut direction: &'static str = "none";
    let mut max_product: u64 = 0;
    let mut max_group: Vec<u8> = vec![];

    for row in digits.grid.iter() {
        for group in group_iter(row.clone(), 4) {
            let product = get_product(&group);
            if product > max_product {
                direction = "Horizontal";
                max_product = product;
                max_group = group;
            }
        }
    }

    for row in digits.transform_vertical().grid.iter() {
        for group in group_iter(row.clone(), 4) {
            let product = get_product(&group);
            if product > max_product {
                direction = "Vertical";
                max_product = product;
                max_group = group;
            }
        }
    }

    for row in digits.transform_diagonal_up().grid.iter() {
        for group in group_iter(row.clone(), 4) {
            let product = get_product(&group);
            if product > max_product {
                direction = "Diagonal Up";
                max_product = product;
                max_group = group;
            }
        }
    }

    for row in digits.transform_diagonal_down().grid.iter() {
        for group in group_iter(row.clone(), 4) {
            let product = get_product(&group);
            if product > max_product {
                direction = "Diagonal Down";
                max_product = product;
                max_group = group;
            }
        }
    }

    println!("max product: {} ({}, from digits {})", max_product, direction, max_group)
}

struct GroupIter {
    index: uint,
    collection: Vec<u8>,
    count: uint,
}

fn get_product(vec: &Vec<u8>) -> u64 {
    vec.iter().fold(1u64, |a, &x| a * x as u64)
}

fn group_iter(vec: Vec<u8>, count: uint) -> GroupIter {
    GroupIter { index: 0, collection: vec, count: count }
}

impl Iterator<Vec<u8>> for GroupIter {
    fn next(&mut self) -> Option<Vec<u8>> {
        let idx = self.index;
        let mut results: Vec<u8> = Vec::new();

        self.index += 1;
        for i in range(idx, idx + self.count) {
            let value = self.collection.as_slice().get(i);
            if value.is_none() {
                return None;
            }
            results.push(*value.unwrap());
        }

        Some(results)
    }
}

struct DigitGrid {
    grid: Vec<Vec<u8>>
}

impl DigitGrid {
    fn transform_vertical(&self) -> DigitGrid {
        let size = 20u;
        let grid = &self.grid;
        let mut new_grid = Vec::new();
        for x in range(0, size) {
            let mut row = Vec::new();
            for y in range(0, size).rev() {
                row.push(*grid.get(y).get(x));
            }
            new_grid.push(row);
        }
        DigitGrid{ grid: new_grid }
    }

    fn transform_diagonal_up(&self) -> DigitGrid {
        let size = 20u;
        let grid = &self.grid;
        let mut new_grid = Vec::new();
        let mut y = size;
        while y != 0 {
            y -= 1;
            let mut xx = 0u;
            let mut yy = y;
            let mut row = Vec::new();
            while yy < size && xx < size {
                row.push(*grid.get(yy).get(xx));
                yy += 1;
                xx += 1;
            }
            new_grid.push(row);
        }

        let mut x = 1u;
        while x < size {
            let mut xx = x;
            let mut yy = 0u;
            let mut row = Vec::new();
            while yy < size && xx < size {
                row.push(*grid.get(yy).get(xx));
                yy += 1;
                xx += 1;
            }
            new_grid.push(row);
            x += 1;
        }

        DigitGrid{ grid: new_grid }
    }

    fn transform_diagonal_down(&self) -> DigitGrid {
        let size = 20i;
        let grid = &self.grid;
        let mut new_grid = Vec::new();
        let mut x = 0i;
        while x < size {
            let mut xx = x;
            let mut yy = 0i;
            let mut row = Vec::new();
            while yy < size && xx >= 0 {
                row.push(*grid.get(yy as uint).get(xx as uint));
                yy += 1;
                xx -= 1;
            }
            new_grid.push(row);
            x += 1;
        }

        let mut y = 1;
        while y < size {
            let mut xx = size - 1;
            let mut yy = y;
            let mut row = Vec::new();
            while yy < size && xx >= 0 {
                row.push(*grid.get(yy as uint).get(xx as uint));
                yy += 1;
                xx -= 1;
            }
            new_grid.push(row);
            y += 1;
        }

        DigitGrid{ grid: new_grid }
    }

}
```