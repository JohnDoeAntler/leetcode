struct Solution {}

fn print_list(head: Option<Box<ListNode>>) {
    let mut current = head;

    while current.is_some() {
        print!("{}, ", current.as_ref().unwrap().val);
        current = current.unwrap().next;
    }

    println!();
}

fn main() {
    // head = [1,2,3,4,5], k = 2
    print_list(Solution::reverse_k_group(Some(
        Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None
                        }))
                    }))
                }))
            }))
        })
    ), 2));

    print_list(Solution::reverse_k_group(Some(
        Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None
                        }))
                    }))
                }))
            }))
        })
    ), 3));
}

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

// --
impl Solution {
    fn find_index(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut current = head;

        for _ in 0..k-1 {
            if current.is_none() {
                return None;
            }

            current = current.unwrap().next;
        }

        current
    }

    pub fn reverse_k_elements(head: Option<Box<ListNode>>, k: i32, next_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // assume head will never be None
        let mut new = ListNode {
            val: head.as_ref().unwrap().val,
            next: next_head,
        };
        let mut current = head;

        for _ in 0..k-1 {
            let next = current.unwrap().next;

            new = ListNode {
                val: next.as_ref().unwrap().val,
                next: Some(Box::new(new))
            };

            current = next;
        };

        Some(Box::new(new))
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // base case 1: if no head
        if head.is_none() {
            return None;
        }
        let head = head.unwrap();

        // base case 2: if there are less than k elements
        let kth = Solution::find_index(Some(head.to_owned()), k);

        if kth.is_none() {
            return Some(head.to_owned());
        };

        // base case 3: if there are >= k elements
        let next_head = Solution::reverse_k_group(kth.to_owned().unwrap().next, k); // fine
                                                                                    //
        // reverse k elements in the current list
        let kth = Solution::reverse_k_elements(Some(head.to_owned()), k, next_head);

        return kth;
    }
}
