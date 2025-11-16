use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

impl Solution {
    fn find_kth(node: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> (Option<i32>, i32) {
        if node.is_none() {
            return (None, 0);
        }

        let (res, left_count) = Self::find_kth(&node.as_ref().unwrap().borrow().left, k);
        if res.is_some() {
            return (res, 0);
        }
        if left_count == k - 1 {
            return (Some(node.as_ref().unwrap().borrow().val), 0);
        }

        let (res, right_count) =
            Self::find_kth(&node.as_ref().unwrap().borrow().right, k - left_count - 1);
        if res.is_some() {
            return (res, 0);
        }

        (None, left_count + right_count + 1)
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::find_kth(&root, k).0.unwrap()
    }
}

struct Solution {}
