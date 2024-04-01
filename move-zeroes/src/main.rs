struct Solution;

fn main() {
    let mut arr = vec![4,2,4,0,0,3,0,5,1,0];
    Solution::move_zeroes(&mut arr);
    println!("{:?}", arr);
}

// --

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut swap_idx = 1;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                while swap_idx < nums.len() && nums[swap_idx] == 0 {
                    swap_idx += 1;
                }

                if swap_idx >= nums.len() {
                    return
                }

                nums.swap(i, swap_idx);
            }

            swap_idx += 1;
        }
    }
}
