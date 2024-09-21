struct Solution;

fn main() {
    println!("{}", Solution::subarray_sum(vec![1,1,1], 2));
    println!("{}", Solution::subarray_sum(vec![1,2,3], 3));
    println!("{}", Solution::subarray_sum(vec![0,0,1,2,0,0], 3));
    println!("{}", Solution::subarray_sum(vec![99,0,0,1,2,0,0], 3));
    // 0 0 1 2
    // 0 0 1 2 0
    // 0 0 1 2 0 0
    // 0 1 2
    // 0 1 2 0
    // 0 1 2 0 0
    // 1 2
    // 1 2 0
    // 1 2 0 0
    println!("{}", Solution::subarray_sum(vec![0,0,0,0], 0));
    // 0 0 0 0 4 + 3 + 3
    // 00 00 00
    // 000 000
    // 0000
    println!("{}", Solution::subarray_sum(vec![1,5,5,5,1], 16));
    println!("{}", Solution::subarray_sum(vec![1], 0));
}

// --
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut ret = 0;
        let mut acc = 0;

        map.insert(0, 1);

        for &e in nums.iter() {
            acc += e;

            if let Some(hit) = map.get(&(acc - k)) {
                ret += *hit;
            }

            map.entry(acc).and_modify(|v| *v += 1).or_insert(1);
        }

        ret
    }
}
