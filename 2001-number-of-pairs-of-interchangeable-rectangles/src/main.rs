struct Solution;

fn main() {
    println!("{}", Solution::interchangeable_rectangles(vec![vec![4,8], vec![3,6], vec![10,20], vec![15,30]]));
    println!("{}", Solution::interchangeable_rectangles(vec![vec![4,5], vec![7,8]]));
}

// --
use std::collections::HashMap;

pub fn gcd(mut n: i32, mut m: i32) -> i32 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      std::mem::swap(&mut m, &mut n);
    }
    m %= n;
  }
  n
}

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut map = HashMap::new();
        let mut ret = 0;

        for e in rectangles.iter().map(|r| {
            let numerator = r[0];
            let denominator = r[1];
            let gcd = gcd(numerator, denominator);

            [numerator / gcd, denominator / gcd]
        }) {
            if let Some(v) = map.get(&e) {
                ret += v;
            };

            map.entry(e).and_modify(|e| *e += 1).or_insert(1);
        };

        ret
    }
}
