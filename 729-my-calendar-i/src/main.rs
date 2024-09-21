fn main() {
    let mut cal = MyCalendar::new();
}

// --

#[derive(Debug)]
struct Interval {
    start: i32,
    end: i32,
}

struct MyCalendar {
    arr: Vec<Interval>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        MyCalendar {
            arr: Vec::new()
        }
    }

    fn insert(&mut self, interval: Interval) {
        let index = self
            .arr
            .binary_search_by_key(&interval.start, |i| i.start);

        match index {
            Ok(i) => self.arr.insert(i, interval),
            Err(i) => self.arr.insert(i, interval),
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        // 1. find the index of the interval that has start > e.start for e in arr
        // 1. find the index of the interval that has end < e.end for e in arr

        let condition = self.arr.len() == 0 || {
            let left = self.arr.iter().rev().position(|e| e.end <= start).map(|e| (self.arr.len() - e - 1) as i32).unwrap_or(-1);
            let right = self.arr.iter().position(|e| e.start >= end).unwrap_or(self.arr.len()) as i32;

            right - left == 1
        };

        if condition {
            self.insert(Interval { start, end });
        };

        condition
    }
}
