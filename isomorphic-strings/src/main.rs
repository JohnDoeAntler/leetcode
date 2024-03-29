struct Solution;

fn main() {
    println!("{}", Solution::is_isomorphic("add".to_string(), "egg".to_string()));
    println!("{}", Solution::is_isomorphic("foo".to_string(), "bar".to_string()));
    println!("{}", Solution::is_isomorphic("paper".to_string(), "title".to_string()));
}

// --

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // assume the len of s and the len of t are always the same
        let mut map1: [u8; 128] = [0; 128];
        let mut map2: [u8; 128] = [0; 128];
        let mut s_iterator = s.chars();
        let mut t_iterator = t.chars();

        loop {
            let s = s_iterator.next();
            let t = t_iterator.next();
            if s.is_none() || t.is_none() {
                break true;
            }
            let s = s.unwrap() as u8;
            let t = t.unwrap() as u8;

            if map1[s as usize] != 0 && map1[s as usize] != t + 1 {
                break false;
            }

            if map2[t as usize] != 0 && map2[t as usize] != s + 1{
                break false;
            }

            map1[s as usize] = t + 1;
            map2[t as usize] = s + 1;
        }
    }
}
