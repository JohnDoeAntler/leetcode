struct Solution; 

fn main() {
    println!("{}", Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
    println!("{}", Solution::is_anagram("rat".to_string(), "car".to_string()));
}

// --
use std::collections::HashMap;

impl Solution {
    pub fn get_map(s: String) -> HashMap<char, i32> {
        let mut map: HashMap<char, i32> = HashMap::new();

        s.chars().for_each(|c| {
            if map.contains_key(&c) {
                *map.get_mut(&c).unwrap() += 1;
            } else {
                map.insert(c, 1);
            }
        });

        map
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let m1 = Solution::get_map(s);
        let m2 = Solution::get_map(t);

        let all_keys = m1.keys().chain(m2.keys()).collect::<Vec<&char>>();

        all_keys.iter().all(|&k| m1.get(k) == m2.get(k))
    }
}
