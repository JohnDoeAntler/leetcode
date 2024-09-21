// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution {}

fn main() {
    let tree123 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    };
    let rc = Rc::new(RefCell::new(tree123));
    println!("result: {}", Solution::is_same_tree(Some(rc.to_owned()), Some(rc.to_owned())));
}

// ---

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() || q.is_none() {
            return false;
        }

        let (p, q) = (p.unwrap(), q.unwrap());

        let result = p.borrow().val == q.borrow().val;

        if !result {
            return false;
        }

        Solution::is_same_tree(p.borrow().left.to_owned(), q.borrow().left.to_owned()) && Solution::is_same_tree(p.borrow().right.to_owned(), q.borrow().right.to_owned())
    }
}
