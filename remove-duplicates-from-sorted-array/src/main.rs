struct Solution;

fn main() {
    let mut vec = vec![1, 1, 2];
    println!("{}", Solution::remove_duplicates(&mut vec));
    println!("{:?}", vec);

    // --

    let mut vec = vec![0,0,1,1,1,2,2,3,3,4];
    println!("{}", Solution::remove_duplicates(&mut vec));
    println!("{:?}", vec);
}

// --

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let mut total = 1;
        let mut swap_idx = 2;

        for idx in 1..nums.len() {
            let previous = nums[idx - 1];
            let current = nums[idx];

            if current <= previous {
                while swap_idx < nums.len() && nums[swap_idx] <= previous {
                    swap_idx += 1;
                }

                if swap_idx >= nums.len() {
                    return total;
                }

                nums[idx] = nums[swap_idx];
            }

            total += 1;
            swap_idx += 1;
        }

        total
    }
}
