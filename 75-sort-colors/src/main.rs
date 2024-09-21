struct Solution;

fn main() {
    let mut arr = vec![2,0,2,1,1,0];
    Solution::sort_colors(&mut arr);
    println!("{:?}", arr);

    let mut arr = vec![2,0,1];
    Solution::sort_colors(&mut arr);
    println!("{:?}", arr);
}

// --

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut map: [u32; 3] = [0; 3];

        nums.iter().for_each(|&e| {
            map[e as usize] += 1;
        });

        let i1 = map[0];
        let i2 = map[1] + i1;
        let i3 = map[2] + i2;

        for i in 0..i1 {
            nums[i as usize] = 0;
        }

        for i in i1..i2 {
            nums[i as usize] = 1;
        }

        for i in i2..i3 {
            nums[i as usize] = 2;
        }
    }
}
