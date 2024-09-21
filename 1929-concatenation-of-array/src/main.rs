struct Solution;

fn main() {
    println!("Hello, world!");
}

// --

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }
}
