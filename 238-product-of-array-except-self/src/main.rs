struct Solution;

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1,2,3,4]));
    println!("{:?}", Solution::product_except_self(vec![-1,1,0,-3,3]));
}

// --

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![1; nums.len()];
        let mut left = 1;

        for (i, &e) in nums.iter().enumerate().take(nums.len() - 1) {
            left *= e;
            ret[i + 1] *= left;
        };

        let mut right = 1;

        for (i, &e) in nums.iter().enumerate().skip(1).rev() {
            right *= e;
            ret[i - 1] *= right;
        };

        ret
    }
}
