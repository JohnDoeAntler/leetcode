struct Solution;

fn main() {
    println!("{}", Solution::largest_number(vec![10, 2]));
    println!("{}", Solution::largest_number(vec![3, 30, 34, 5, 9]));
    // println!("{}", Solution::largest_number(vec![3, 91, 92, 5, 9]));
    // println!("{}", Solution::largest_number(vec![3, 91, 998, 5, 99]));
    println!("{}", Solution::largest_number(vec![34323, 3432]));
    println!("{}", Solution::largest_number(vec![432, 43243]));
    println!("{}", Solution::largest_number(vec![0, 0, 0, 0]));
}

// #[derive(Debug, Clone)]
// struct Digit {
//     count: u64,
//     children: Vec<Digit>,
// }
//
// impl Digit {
//     pub fn new () -> Self {
//         Self {
//             count: 0,
//             children: vec![],
//         }
//     }
// }
//
// impl Solution {
//
//     pub fn largest_number(nums: Vec<i32>) -> String {
//         let mut root = Digit {
//             count: 0,
//             children: vec![],
//         };
//
//         for &num in nums.iter() {
//             let mut vec = vec![];
//             let mut digit = num;
//
//             while digit > 0 {
//                 vec.push(digit % 10);
//                 digit /= 10;
//             }
//
//             let mut current = &mut root;
//
//             while let Some(digit) = vec.pop() {
//                 if current.children.is_empty() {
//                     current.children = vec![Digit::new(); 10];
//                 }
//                 current = &mut current.children[digit as usize];
//             };
//
//             current.count += 1;
//         }
//
//         // beautify the tree printing
//         let mut ret = String::new();
//
//         for (i, child) in root.children.iter().enumerate().rev() {
//             Solution::helper(child, &mut ret, i.to_string(), i);
//             Solution::helper(child, &mut ret, i.to_string(), i);
//         }
//
//         ret
//     }
//
//     pub fn helper(node: &Digit, ret: &mut String, prefix: String, current: usize) {
//         let mut e = node.children.iter().enumerate().rev();
//
//         for (i, child) in e.by_ref().take(9 - current) {
//             Solution::helper(child, ret, format!("{}{}", prefix, i.to_string()), current);
//         }
//
//         ret.push_str(&prefix.repeat(node.count as usize));
//
//         for (i, child) in e {
//             Solution::helper(child, ret, format!("{}{}", prefix, i.to_string()), current);
//         }
//     }
// }

// --

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strs = nums.iter().map(|e| e.to_string()).collect::<Vec<_>>();

        strs.sort_by(|a, b| {
            let ab = a.clone() + &b;
            let ba = b.clone() + &a;

            ba.cmp(&ab)
        });

        if strs[0] == "0" {
            return "0".to_string();
        }

        strs.join("").chars().collect()
    }
}
