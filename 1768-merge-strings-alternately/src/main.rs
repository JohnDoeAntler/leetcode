struct Solution;

fn main() {
    println!("{}", Solution::merge_alternately("abc".to_string(), "pqr".to_string()));
    println!("{}", Solution::merge_alternately("ab".to_string(), "pqrs".to_string()));
    println!("{}", Solution::merge_alternately("abcd".to_string(), "pq".to_string()));
}

// --

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut str = String::new();
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();

        for _ in 0..std::cmp::min(word1.len(), word2.len()) {
            let (a, b) = (iter1.next().unwrap(), iter2.next().unwrap());
            str.push(a);
            str.push(b);
        }

        for e in iter1 {
            str.push(e);
        }

        for e in iter2 {
            str.push(e);
        }

        str
    }
}

