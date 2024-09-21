struct Solution;

fn main() {
    println!("Hello, world!");
    println!("s1: {:?}", Solution::group_anagrams(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()]));
}

// --

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<_>> = HashMap::new();

        for e in strs.into_iter() {
            let mut m: [u8; 26] = [0; 26];
            e.chars().for_each(|c| m[(c as u8 - 'a' as u8) as usize] += 1);

            map.entry(m).or_insert(vec![]).push(e);
        };

        map.values().map(|v| v.clone()).collect()
    }
}
