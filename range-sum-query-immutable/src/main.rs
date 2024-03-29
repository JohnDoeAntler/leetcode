fn main() {
    let arr = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    println!("{}", arr.sum_range(0, 2));
    println!("{}", arr.sum_range(2, 5));
    println!("{}", arr.sum_range(0, 5));
}

struct NumArray {
    sum: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut ret = vec![0; nums.len() + 1];
        let mut sum = 0;

        for (i, &e) in nums.iter().enumerate().rev() {
            sum += e;
            ret[i] = sum;
        }

        NumArray {
            sum: ret
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return self.sum[left as usize] - self.sum[(right + 1) as usize];
    }
}

