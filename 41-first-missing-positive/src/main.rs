struct Solution;

fn main() {
    println!("{}", Solution::first_missing_positive(vec![1,2,0]));
    println!("{}", Solution::first_missing_positive(vec![3,4,-1,1]));
    println!("{}", Solution::first_missing_positive(vec![7,8,9,10,11]));
    println!("{}", Solution::first_missing_positive(vec![1]));
    println!("{}", Solution::first_missing_positive(vec![1,1]));
}

//--

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let nums = {
            let mut nums = nums;

            nums.iter_mut().filter(|e| **e <= 0).for_each(|e| *e = i32::MAX);

            nums
        };

        let tmp = {
            let mut tmp = nums;

            for i in 0..tmp.len() {
                let current = tmp[i];
                if current == i32::MIN {
                    continue;
                }

                let abs = current.abs();
                let idx = abs - 1;

                if !(idx >= 0 && idx < tmp.len() as i32) {
                    continue;
                }

                let idx = idx as usize;
                tmp[idx] = -tmp[idx].abs();
            }

            tmp
        };

        println!("{:?}", tmp);

        tmp.iter().enumerate().find(|(_, &e)| e >= 0).map(|(i, &_)| i as i32).unwrap_or(tmp.len() as i32) +1
    }
}
