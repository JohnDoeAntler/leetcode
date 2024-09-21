struct Solution;

fn main() {
    println!("{}", Solution::is_subsequence("abc".to_string(), "abhgdc".to_string()));
    println!();
    println!("{}", Solution::is_subsequence("axc".to_string(), "abhgdc".to_string()));
    println!();
    println!("{}", Solution::is_subsequence("abc".to_string(), "abc".to_string()));
    println!();
    println!("{}", Solution::is_subsequence("abc".to_string(), "ab".to_string()));
    println!();
    println!("{}", Solution::is_subsequence("ab".to_string(), "abc".to_string()));
    println!();
    println!("{}", Solution::is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()));
}

// --

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut last_index = 0;

        for c in s.chars() {
            let idx = t[last_index..t.len()].chars().position(|e| e == c);

            if idx.is_none() {
                return false;
            }

            last_index += idx.unwrap() + 1;
        };

        true
    }
}
