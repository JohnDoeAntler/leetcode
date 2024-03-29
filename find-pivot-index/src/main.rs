struct Solution;

fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::pivot_index(vec![1,7,3,6,5,6]));
    println!("{:?}", Solution::pivot_index(vec![1,2,3]));
    println!("{:?}", Solution::pivot_index(vec![2,1,-1]));
}

// --

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut acc = 0;

        for (i, &e) in nums.iter().enumerate() {
            let filtered = sum - e;
            if filtered % 2 == 0 && filtered / 2 == acc {
                return i as i32
            }
            acc += e;
        }

        -1
    }
}
