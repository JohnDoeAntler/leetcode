struct Solution {}

fn main() {
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 2));
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 9));
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], -1));
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], -2));
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 12));
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 13));
    println!("{}", Solution::search(vec![1,2,3], 3));
    println!("{}", Solution::search(vec![1,2,3,4], 4));
    println!("{}", Solution::search(vec![1,2,3], 1));
    println!("{}", Solution::search(vec![1,2,3,4], 1));
}

// ---

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let count = &nums.len() - 1;
        return Solution::helper(nums, target, 0, count);
    }

    pub fn helper(nums: Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        let mid = (left + right) / 2;


        if nums[mid] == target {
            mid as i32
        } else if left == right && nums[mid] != target {
            -1
        } else if target > nums[mid] {
            Solution::helper(nums, target, mid + 1, right)
        } else {
            Solution::helper(nums, target, left, mid)
        }
    }
}

