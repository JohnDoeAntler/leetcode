struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn print_list(head: Option<Box<ListNode>>) {
    let mut head = head;
    while head.is_some() {
        let node = head.unwrap();
        print!("{} ", node.val);
        head = node.next;
    }
    println!();
}

fn main() {
    let ret1 = Solution::swap_pairs(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }))
    })));

    let ret2 = Solution::swap_pairs(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None
            }))
        }))
    })));

    let ret3 = Solution::swap_pairs(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: None
        }))
    })));

    let ret4 = Solution::swap_pairs(Some(Box::new(ListNode {
        val: 1,
        next: None
    })));

    let ret5 = Solution::swap_pairs(None);
    print_list(ret1);
    print_list(ret2);
    print_list(ret3);
    print_list(ret4);
    print_list(ret5);
}

// --

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut first) => {
                match first.next {
                    None => Some(first),
                    Some(mut second) => {
                        let next_head = second.next.take();
                        first.next = Solution::swap_pairs(next_head);
                        second.next = Some(first);
                        Some(second)
                    }
                }
            }
        }
    }
}
