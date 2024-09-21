struct Solution {}

fn main() {
    println!("{:?}", Solution::find_matrix(vec![1,3,4,1,2,3,1]));
}

// --

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // [0; nums.len()]
        let mut map = vec![0; nums.len()];

        nums.iter().for_each(|e| {
            let r = map.get_mut((*e - 1) as usize).unwrap();
            *r += 1;
        });

        let mut ret = Vec::new();

        while map.iter().any(|e| *e != 0) {
            let mut current_row = Vec::new();

            map.iter_mut().enumerate().for_each(|(idx, e)| {
                if *e != 0 {
                    current_row.push((idx + 1) as i32);
                    *e -= 1;
                }
            });

            ret.push(current_row);
        }

        ret
    }
}

