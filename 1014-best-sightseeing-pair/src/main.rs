struct Solution;

fn main() {
    println!("{}", Solution::max_score_sightseeing_pair(vec![8,1,5,2,6]));
    println!("{}", Solution::max_score_sightseeing_pair(vec![9,0,9,0,10,0,10]));
    // 16
    //                                                           14
    //                                                       -1 0 13 0 10 
}

// ---

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        // values are guaranteed to have at least 2 elements
        let mut accumlated = values[0];
        let mut max = values[0];

        for e in values.iter().skip(1) {
            max -= 1;
            accumlated = std::cmp::max(accumlated, max + *e);
            max = std::cmp::max(max, *e);
        };

        accumlated
    }
}
