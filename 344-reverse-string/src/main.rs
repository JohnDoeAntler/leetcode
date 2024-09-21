struct Solution {}

fn main() {
    let mut testcase1 = vec!['h', 'e', 'l', 'l', 'o'];
    let mut testcase2 =vec!['H', 'a', 'n', 'n', 'a', 'h'];

    Solution::reverse_string(&mut testcase1);
    Solution::reverse_string(&mut testcase2);

    println!("{:?}", testcase1);
    println!("{:?}", testcase2);
}

// --

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        Solution::helper(s, 0, s.len() - 1);
    }

    pub fn helper(s: &mut Vec<char>, left: usize, right: usize) {
        // base case
        if left >= right {
            return;
        }

        let tmp = s[left];
        s[left] = s[right];
        s[right] = tmp;

        Solution::helper(s, left + 1, right - 1);
    }
}
