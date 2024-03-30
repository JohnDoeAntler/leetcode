struct Solution;

fn main() {
    println!("{}", Solution::longest_palindrome_subseq("bbbab".to_string()));
    println!("{}", Solution::longest_palindrome_subseq("cbbd".to_string()));
}

// --

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut mem = vec![vec![0; s.len() + 1]; s.len() + 1];
        let rev = s.chars().rev().collect::<String>();

        for (i_idx, i) in s.chars().enumerate() {
            for (j_idx, j) in rev.chars().enumerate() {
                if i == j {
                    mem[i_idx + 1][j_idx + 1] = mem[i_idx][j_idx] + 1;
                } else {
                    mem[i_idx + 1][j_idx + 1] = std::cmp::max(
                        mem[i_idx + 1][j_idx],
                        mem[i_idx][j_idx + 1],
                    );
                }
            };
        };

        mem[s.len()][s.len()]
    }
}
