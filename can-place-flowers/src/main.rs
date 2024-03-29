struct Solution;

fn main() {
    println!("{}", Solution::can_place_flowers(vec![1,0,0,0,1], 1));
    println!("{}", Solution::can_place_flowers(vec![1,0,0,0,0,1], 2));
    println!("{}", Solution::can_place_flowers(vec![0,0,1,0,1], 1));
    println!("{}", Solution::can_place_flowers(vec![1,0,0,0,1,0,0], 2));


}

// --

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut planted = 0;
        let mut was_empty = false;
        let mut first_empty_index = 0;
        let k = [1,0].iter().chain(&flowerbed).chain(&[0, 1]);

        for (i, &e) in k.enumerate() {
            if was_empty && e == 1 {
                was_empty = false;
                planted += (i - first_empty_index - 1) / 2;
            } else if !was_empty && e == 0 {
                first_empty_index = i;
                was_empty = true;
            }
        }

        planted as i32 >= n
    }
}
