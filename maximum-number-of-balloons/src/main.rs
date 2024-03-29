use std::cmp::min;

struct Solution;

fn main() {
    println!("Hello, world!");
    println!("{}", Solution::max_number_of_balloons("nlaebolko".to_string()));
    println!("{}", Solution::max_number_of_balloons("loonbalxballpoon".to_string()));
}

// --

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map: [i32; 26] = [0; 26];

        for e in text.chars() {
            map[(e as u8 - 'a' as u8) as usize] += 1;
        }

        let b = map[('b' as u8 - 'a' as u8) as usize];
        let a = map[('a' as u8 - 'a' as u8) as usize];
        let l = map[('l' as u8 - 'a' as u8) as usize] / 2;
        let o = map[('o' as u8 - 'a' as u8) as usize] / 2;
        let n = map[('n' as u8 - 'a' as u8) as usize];

        min(b, min(a, min(l, min(o, n))))
    }
}
