struct Solution;

fn main() {
    let mut a = vec![1,2,3,0,0,0,];
    let m = 3;
    let mut b = vec![2,5,6];
    let n = 3;
    println!("{:?}", Solution::merge(&mut a, m, &mut b, n));
}

// --

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx1 = m - 1;
        let mut idx2 = n - 1;
        let modifying_idx = m + n;

        for i in (0..modifying_idx as usize).rev() {
            let j = if idx1 >= 0 { nums1[idx1 as usize] } else { i32::MIN };
            let k = if idx2 >= 0 { nums2[idx2 as usize] } else { i32::MIN };

            if j > k {
                nums1[i] = j;
                idx1 -= 1;
            } else {
                nums1[i] = k;
                idx2 -= 1;
            }
        }
    }
}
