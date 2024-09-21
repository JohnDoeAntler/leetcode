struct Solution;

fn main() {
    println!("{}", Solution::valid_palindrome("aba".to_string()));
    println!("{}", Solution::valid_palindrome("abca".to_string()));
    println!("{}", Solution::valid_palindrome("abc".to_string()));
}

// --
impl Solution {
    pub fn helper(s: &Vec<char>, left: usize, right: usize, tolerance: usize) -> bool {
        let mut left = left;
        let mut right = right;

        while left < right {
            if s[left] != s[right] {
                if tolerance > 0 {
                    return Solution::helper(s, left + 1, right, tolerance - 1) || Solution::helper(s, left, right - 1, tolerance - 1);
                } else {
                    return false;
                }
            }

            left += 1;
            right -= 1;
        }

        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let arr = s.chars().collect::<Vec<_>>();
        Solution::helper(&arr, 0, arr.len() - 1, 1)
    }
}
