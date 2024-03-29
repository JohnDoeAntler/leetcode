struct Solution;

fn main() {
    println!("{}", Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()));
    println!("{}", Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()));
    println!("{}", Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()));
    println!("{}", Solution::word_pattern("ab".to_string(), "dog cat cat dog".to_string()));
    println!("{}", Solution::word_pattern("abba".to_string(), "dog cat".to_string()));
    println!("{}", Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()));
    println!();
    println!("{}", Solution::word_pattern("he".to_string(), "unit".to_string()));
}
// --

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map: Vec<Option<String>> = vec![None; 26];
        let mut chars = pattern.chars();
        let mut iterator = s.split_whitespace();

        loop {
            let char = chars.next();
            let current = iterator.next();

            // pattern.len() == s.len()
            if char.is_none() || current.is_none() {
                return char.is_none() && current.is_none();
            }

            let (char, current) = (char.unwrap(), current.unwrap());

            let idx = (char as u8 - 'a' as u8) as usize;
            let value = map[idx].as_ref();

            if let Some(str) = value {
                if current != str {
                    return false;
                }
            } else {
                if map.iter().any(|e| e.as_ref().is_some_and(|e| e == current)) {
                    return false;
                }
                map[idx] = Some(current.to_string());
            }
        }
    }
}
