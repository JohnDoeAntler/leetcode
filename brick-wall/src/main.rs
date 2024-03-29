struct Solution;

fn main() {
    println!(
        "{}",
        Solution::least_bricks(vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ])
    );

    println!(
        "{}",
        Solution::least_bricks(vec![vec![1], vec![1], vec![1],])
    );
}

// --
use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut max_frequency = 0;

        wall.iter().for_each(|row| {
            let mut offset = 0;

            row.iter().take(row.len() - 1).for_each(|&brick| {
                offset += brick;
                let current = map.entry(offset).or_insert(0);
                *current += 1;

                max_frequency = std::cmp::max(max_frequency, *current)
            });
        });

        wall.len() as i32 - max_frequency
    }
}
