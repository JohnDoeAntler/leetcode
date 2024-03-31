struct Solution;

fn main() {
    println!("{}", Solution::get_permutation(3, 3));
    println!("{}", Solution::get_permutation(4, 9));
    println!("{}", Solution::get_permutation(3, 1));
    println!("{}", Solution::get_permutation(3, 6));
    println!("{}", Solution::get_permutation(5, 120));
}

// --

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut k = k - 1;

        let mut chars = (0..n).map(|e| e + 1).map(|e| e.to_string()).collect::<Vec<_>>();
        let mut ret = String::new();
        let mut n = n - 1;
        //             0, 1, 2, 3,  4,   5,   6,    7,     8
        let fac_map = [1, 1, 2, 6, 24, 120, 720, 5040, 40320];

        while !chars.is_empty() {
            // get idx
            let idx = k / fac_map[n as usize];
            // get reminder
            k = k % fac_map[n as usize];
            // push char
            ret.push_str(chars.remove(idx as usize).as_str());
            // minus n
            n -= 1;
        }

        ret
    }
}
