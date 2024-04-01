struct Solution;

fn main() {
    println!("Hello, world!");
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", Solution::max_profit(vec![5,2,7,1]));
}

// --

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut ret = 0;

        for &e in prices.iter() {
            min = std::cmp::min(min, e);
            ret = std::cmp::max(ret, e - min);
        }

        ret
    }
}
