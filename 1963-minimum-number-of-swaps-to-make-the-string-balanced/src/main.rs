struct Solution;

fn main() {
    println!("{}", Solution::min_swaps("[[]]]][[".to_string()));
    println!("{}", Solution::min_swaps("][][".to_string()));
    println!("{}", Solution::min_swaps("]]][[[".to_string()));
    println!("{}", Solution::min_swaps("[]".to_string()));
}

// --

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut counter = 0;
        let mut ret = 0;

        s.chars().for_each(|e| {
            match e {
                '[' => counter += 1,
                ']' => {
                    if counter == 0 {
                        ret += 1;
                        counter += 1;
                    } else {
                        counter -= 1;
                    }
                }
                _ => unreachable!(),
            };
        });

        ret
    }
}

