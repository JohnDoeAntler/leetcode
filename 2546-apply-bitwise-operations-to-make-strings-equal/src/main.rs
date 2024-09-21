struct Solution {}

fn main() {
    println!("{:?}", Solution::make_strings_equal("1010".to_string(), "0110".to_string()));
    println!("{:?}", Solution::make_strings_equal("11".to_string(), "00".to_string()));
}

// ---

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        return s == target || !s.chars().into_iter().all(|c| c == '0') && !target.chars().into_iter().all(|c| c == '0')
    }
}
