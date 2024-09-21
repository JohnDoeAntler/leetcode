struct Solution;

fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![1,1,1,2,2,3], 2));
    println!("{:?}", Solution::top_k_frequent(vec![1], 1));
}

// --
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for &i in nums.iter() {
            *map.entry(i).or_insert(0) += 1;
        }

        let mut pairs = map.iter().collect::<Vec<(&i32, &i32)>>();
        pairs.sort_by(|(_, &a), (_, b)| a.partial_cmp(b).unwrap());

        pairs.iter().rev().take(k as usize).map(|(&a, _)| a).collect()
    }
}
