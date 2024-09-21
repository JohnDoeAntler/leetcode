struct Solution;

fn main() {
    println!("{:?}", Solution::next_greater_element(vec![4,1,2], vec![1,3,4,2]));
    println!("{:?}", Solution::next_greater_element(vec![2,4], vec![1,2,3,4]));
}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![-1];
        let mut ret = vec![-1; nums1.len()];

        for &e in nums2.iter().rev() {
            let whatever = nums1.iter().position(|&k| k == e);
            if let Some(k) = whatever {
                ret[k] = stack.iter().rev().find(|&&k| k > e).map(|&k| k).unwrap_or(-1);
            }

            while !stack.is_empty() && *stack.last().unwrap() < e {
                stack.pop();
            }
            stack.push(e);
        }

        ret
    }
}
