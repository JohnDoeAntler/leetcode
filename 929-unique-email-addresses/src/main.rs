struct Solution;

fn main() {
    println!("Hello, world!");
    println!("{}", Solution::num_unique_emails(vec!["test.email+alex@leetcode.com".to_string(),"test.e.mail+bob.cathy@leetcode.com".to_string(),"testemail+david@lee.tcode.com".to_string()]));
    println!("{}", Solution::num_unique_emails(vec!["a@leetcode.com".to_string(),"b@leetcode.com".to_string(),"c@leetcode.com".to_string()]));
}

// --

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn extract_local_name_and_domain(str: String) -> ([u8; 26], String) {
        let mut arr: [u8; 26] = [0; 26];
        let mut flag = false;
        let mut iterator = str.chars();

        for c in iterator.by_ref() {
            if c == '@' {
                break;
            } else 
            if flag || c == '.' {
                continue;
            } else
            if c == '+' {
                flag = true
            } else {
                arr[(c as u8 - 'a' as u8) as usize] += 1;
            }
        }

        return (arr, iterator.collect::<String>());
    }

    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut map: HashMap<String, HashSet<[u8; 26]>> = HashMap::new();
        // string => hashset<string>

        for i in emails.into_iter() {
            let (local_name, domain) = Solution::extract_local_name_and_domain(i);
            map.entry(domain).or_insert(HashSet::new()).insert(local_name);
        };

        map.values().map(|set| set.len()).sum::<usize>() as i32
    }
}
