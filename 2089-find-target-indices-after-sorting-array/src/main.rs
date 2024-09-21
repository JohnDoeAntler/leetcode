struct Solution;

fn main() {
    println!("{:?}", Solution::target_indices(vec![1,2,5,2,3], 2));
    println!("{:?}", Solution::target_indices(vec![1,2,5,2,3], 3));
}

// --

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums = {
            let mut nums = nums;
            nums.sort();
            nums
        };

        nums.iter().enumerate().filter(|(_, &e)| e == target).map(|(i, _)| i as i32).collect()
    }
}
