struct Solution {}

fn main() {
    println!("{:?}", Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10));
    println!("{:?}", Solution::pivot_array(vec![-3, 4, 3, 2], 2));
}

// ---

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut left = Vec::new();
        let mut center = Vec::new();
        let mut right = Vec::new();

        nums.into_iter().for_each(|e| {
            if e < pivot {
                left.push(e);
            } else if e == pivot {
                center.push(e);
            } else {
                right.push(e);
            }
        });

        left.append(&mut center);
        left.append(&mut right);

        left.into()
    }
}
