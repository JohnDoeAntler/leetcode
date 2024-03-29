struct Solution;

fn main() {
    println!("{}", Solution::length_of_last_word("Hello, world".to_string()));
    println!("{}", Solution::length_of_last_word("   fly me   to   the moon  ".to_string()));
    println!("{}", Solution::length_of_last_word("luffy is still joyboy".to_string()));
}

// --

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split_whitespace().last().unwrap().len() as i32
    }
}
