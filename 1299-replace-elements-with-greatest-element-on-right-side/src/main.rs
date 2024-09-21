struct Solution;

fn main() {
    println!("Hello, world! {:?}", Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]));
}

// --

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut max_idx = 0;
        let mut ret: Vec<i32> = Vec::with_capacity(arr.len());

        for i in 1..arr.len() {
            if max_idx < i {
                max_idx = arr.iter().enumerate().skip(i).max_by_key(|(_, &e)| e).map(|(i, &_)| i).unwrap();
            }

            ret.push(arr[max_idx]);
        };

        ret.push(-1);
        ret
    }
}
