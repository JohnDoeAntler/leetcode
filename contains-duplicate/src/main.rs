use std::collections::HashSet;

struct Solution;

fn main() {
    println!("{}", Solution::contains_duplicate(vec![1,2,3,1]));
    println!("{}", Solution::contains_duplicate(vec![1,2,3,4]));
    println!("{}", Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
}

// --

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for e in nums.iter() {
            if set.contains(e) {
                return true;
            }
            set.insert(e);
        }

        return false;
    }
}
