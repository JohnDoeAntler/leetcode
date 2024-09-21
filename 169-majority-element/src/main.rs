struct Solution;

fn main() {
    println!("Hello, world!");
    println!("{}", Solution::majority_element(vec![3, 2, 3]));
    println!("{}", Solution::majority_element(vec![2,2,1,1,1,2,2]));
}
// --

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut num = 0;

        for &e in nums.iter() {
            if e == num {
                count += 1;
            } else if count == 0 {
                num = e;
                count += 1;
            } else {
                count -= 1;
            }
           
        }

        num
    }
}
