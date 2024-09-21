struct Solution;

fn main() {
    println!("{}", Solution::digit_sum("11111222223".to_string(), 3));
    println!("{}", Solution::digit_sum("3465".to_string(), 3));
    println!("{}", Solution::digit_sum("135".to_string(), 3));
    println!("{}", Solution::digit_sum("00000000".to_string(), 3));
}

// --

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        if s.len() as i32 <= k {
            return s;
        };

        Solution::digit_sum(
            s
                .chars()
                .collect::<Vec<char>>()
                .chunks(k as usize)
                .map(|chunk|
                    chunk
                    .into_iter()
                    .map(|e| e
                         .to_digit(10)
                         .unwrap()
                    )
                    .sum::<u32>(),
                )
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(""),
            k
        )
    }
}
