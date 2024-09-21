struct Solution;

fn main() {
    println!("{}", Solution::longest_consecutive(vec![100,4,200,1,3,2]));
    println!("{}", Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]));
}

// --
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut max = 0;

        for &i in set.iter() {
            // if it is the the start of the sequence, simply skip
            if set.contains(&(i - 1)) {
                continue;
            }

            max = std::cmp::max(max, (i..).take_while(|e| set.contains(e)).count());
        }

        max as i32
    }
}
