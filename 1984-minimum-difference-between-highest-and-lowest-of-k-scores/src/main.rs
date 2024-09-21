struct Solution;

fn main() {
    println!("{}", Solution::minimum_difference(vec![90], 1));
    println!("{}", Solution::minimum_difference(vec![9,4,1,7], 2));
    println!("{}", Solution::minimum_difference(vec![1,2,9999,2001,22002,32003], 3));
}

// --

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut ret = i32::MAX;

        for i in (k - 1) as usize..nums.len() {
            ret = std::cmp::min(ret, nums[i] - nums[i + 1 - k as usize]);
        }

        ret
    }
}
