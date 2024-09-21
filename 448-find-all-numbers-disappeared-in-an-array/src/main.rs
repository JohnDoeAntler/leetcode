struct Solution;

fn main() {
    println!("{:?}", Solution::find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]));
    println!("{:?}", Solution::find_disappeared_numbers(vec![1,1]));
}

// --

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map = vec![0; nums.len()];

        for &e in nums.iter() {
            map[(e - 1) as usize] += 1;
        }

        map.iter().enumerate().filter(|(i, &e)| e == 0).map(|(i, _)| (i + 1) as i32).collect::<Vec<_>>()
    }
}
