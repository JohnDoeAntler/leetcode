struct Solution;

fn main() {
    println!("{}", Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}", Solution::max_profit(vec![1,2,3,4,5]));
    println!("{}", Solution::max_profit(vec![7,6,4,3,1]));
}

// --

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for trend in prices.windows(2) {
            if trend[1] > trend[0] {
                profit += trend[1] - trend[0];
            }
        }

        profit
    }
}
