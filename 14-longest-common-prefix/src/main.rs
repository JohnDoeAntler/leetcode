struct Solution;

fn main() {
    println!("ret: {}", Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
}

// --

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut iterators = strs.iter().map(|s| s.chars()).collect::<Vec<_>>();
        let mut ret = String::new();

        loop {
            let mut chars = iterators.iter_mut().map(|chars| chars.next());
            let first = chars.next().unwrap();

            let first = first;

            if first.is_none() {
                return ret;
            }

            let first = first.unwrap();

            loop {
                let k = chars.next();

                if k.is_none() {
                    break;
                }

                let k = k.unwrap();

                if k.is_none() {
                    return ret;
                }

                if k.unwrap() != first {
                    return ret;
                }
            }

            ret.push(first);
        }
    }
}
