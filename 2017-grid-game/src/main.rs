struct Solution;

fn main() {
    println!("{}", Solution::grid_game(vec![vec![2,5,4], vec![1,5,1]]));
    println!("{}", Solution::grid_game(vec![vec![3,3,1], vec![8,5,2]]));
    println!("{}", Solution::grid_game(vec![vec![1,3,1,15], vec![1,3,3,1]]));
    println!("{}", Solution::grid_game(vec![vec![20,3,20,17,2,12,15,17,4,15], vec![20,10,13,14,15,5,2,3,14,3]]));
}

// --

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        // constants
        let grid: Vec<Vec<i64>> = grid.iter().map(|row| row.iter().map(|&num| num as i64).collect()).collect();
        let width = grid[0].len();

        let mut prefix = vec![0; width];
        let mut suffix = vec![0; width];

        for i in 1..width {
            prefix[i] = grid[1][i - 1] + prefix[i - 1];
            suffix[width - i - 1] = grid[0][width - i] + suffix[width - i];
        };

        prefix.iter().zip(suffix.iter()).map(|(&a, &b)| std::cmp::max(a, b)).min().unwrap_or(0)
    }

}
