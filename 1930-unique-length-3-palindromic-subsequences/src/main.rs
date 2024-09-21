struct Solution;

fn main() {
    println!("{}", Solution::count_palindromic_subsequence("aabca".to_string()));
    println!("{}", Solution::count_palindromic_subsequence("adc".to_string()));
    println!("{}", Solution::count_palindromic_subsequence("bbcbaba".to_string()));
}

// --
use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut prefix: Vec<[bool; 26]> = vec![[false; 26]; s.len()];
        let mut suffix: Vec<[bool; 26]> = vec![[false; 26]; s.len()];

        let mut current: [bool; 26] = [false; 26];

        s.chars().enumerate().take(s.len() - 2).for_each(|(i, e)| {
            let normalized = (e as u8 - 'a' as u8) as usize;
            current[normalized] = true;
            prefix[i] = current;
        });

        current = [false; 26];

        s.chars().rev().enumerate().take(s.len() - 2).for_each(|(i, e)| {
            let normalized = (e as u8 - 'a' as u8) as usize;
            current[normalized] = true;
            suffix[s.len() - i - 1] = current;
        });

        let mut set = HashSet::new();

        s.chars().enumerate().take(s.len() - 1).skip(1).for_each(|(i, e)| {
            prefix[i - 1].iter()
                .zip(suffix[i + 1])
                .enumerate()
                .filter(|&(_, (&p, s))| {
                    p && s
                })
                .for_each(|(i, _)| {
                    let k = (i as u8 + 'a' as u8) as char;
                    set.insert([k, e, k]);
                });
        });

        set.len() as i32
    }
}
