struct Solution;

fn main() {
    println!("{:?}", Solution::generate(1));
    println!("{:?}", Solution::generate(2));
    println!("{:?}", Solution::generate(3));
    println!("{:?}", Solution::generate(5));
}

// --

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![1]];

        for i in 1..num_rows {
            let previous_row = ret.last().unwrap();
            let mut row = Vec::new();

            for j in 0..i+1 {
                if j == 0 || i == j {
                    // first element or last element
                    row.push(1);
                } else {
                    let left = previous_row[(j - 1) as usize];
                    let right = previous_row[j as usize];
                    row.push(left + right);
                }
            }

            ret.push(row);
        }

        ret
    }
}
