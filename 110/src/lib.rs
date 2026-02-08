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
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn balanced(node: &Option<Rc<RefCell<TreeNode>>>, curr_height: usize) -> (bool, usize) {
        if node.is_none() {
            return (true, curr_height);
        } else {
            let (left_balanced, left_max) =
                Self::balanced(&node.as_ref().unwrap().borrow().left, curr_height + 1);
            let (right_balanced, right_max) =
                Self::balanced(&node.as_ref().unwrap().borrow().right, curr_height + 1);
            return (
                left_balanced && right_balanced && left_max.abs_diff(right_max) <= 1,
                left_max.max(right_max),
            );
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::balanced(&root, 0).0
    }
}

struct Solution {}
