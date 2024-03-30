struct Solution;

fn main() {
    println!("{:?}", Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()));
    println!("{:?}", Solution::find_anagrams("abab".to_string(), "ab".to_string()));
}

// --

impl Solution {
    pub fn to_arr(test: &[char]) -> [i32; 26] {
        let mut ret = [0; 26];

        test.iter().for_each(|&e| {
            ret[(e as u8 - 'a' as u8) as usize] += 1;
        });

        ret
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let tmp = s.chars().collect::<Vec<_>>();
        let window_size = p.len();
        let p = p.chars().fold([0; 26], |mut a, b| {
            a[(b as u8 - 'a' as u8) as usize] += 1;
            a
        });

        let mut ret = vec![];

        for (i, e) in tmp.windows(window_size).enumerate() {
            if Solution::to_arr(e) == p {
                ret.push(i as i32);
            }
        };

        ret
    }
}
