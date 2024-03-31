struct Solution;

fn main() {
    println!("{}", Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    println!("{}", Solution::max_sub_array(vec![1]));
    println!("{}", Solution::max_sub_array(vec![5,4,-1,7,8]));
}

// --
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ret = i32::MIN;

        for &e in nums.iter() {
            sum = max(sum + e, e);
            ret = max(ret, sum);
        }

        ret
    }
}
