struct Solution;

fn list(arr: Vec<i32>, len: i32) {
    arr.iter().for_each(|e| println!("{}", e));
    println!("end: {}\n", len);
}

fn main() {
    let mut arr = vec![3,2,2,3];
    let len = Solution::remove_element(&mut arr, 3);
    list(arr, len);

    let mut arr = vec![0,1,2,2,3,0,4,2];
    let len = Solution::remove_element(&mut arr, 2);
    list(arr, len);

    let mut arr = vec![2,2,2];
    let len = Solution::remove_element(&mut arr, 2);
    list(arr, len);

    let mut arr = vec![2];
    let len = Solution::remove_element(&mut arr, 3);
    list(arr, len);
}

// --

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            if nums[left] == val {
                while nums[right] == val {
                    if left == right {
                        return left as i32;
                    }

                    right -= 1;
                }

                nums.swap(left, right);
            }

            left += 1;
        }

        left as i32
    }
}
