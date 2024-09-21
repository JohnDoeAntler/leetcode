struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    println!("{}", Solution::is_palindrome("race a car".to_string()));
    println!("{}", Solution::is_palindrome(" ".to_string()));
    println!("{}", Solution::is_palindrome("0P".to_string()));
}

// --

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let vec = s
            .to_lowercase()
            .chars()
            .filter(|&e| e.is_alphanumeric())
            .collect::<Vec<char>>();

        for (i, &e) in vec.iter().enumerate().take(vec.len() / 2) {
            if vec[vec.len() - i - 1] != e {
                return false;
            }
        }

        true
    }
}
