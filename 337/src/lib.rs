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

    fn _from_list(arr: &[Option<i32>], idx: usize) -> Option<Rc<RefCell<Self>>> {
        println!("idx: {}", idx);
        if idx >= arr.len() || arr[idx].is_none() {
            return None;
        }

        let mut curr = Self::new(arr[idx].unwrap());
        let left = Self::_from_list(arr, 2 * idx + 1);
        let right = Self::_from_list(arr, 2 * idx + 2);
        let _ = std::mem::replace(&mut curr.left, left);
        let _ = std::mem::replace(&mut curr.right, right);

        return Some(Rc::new(RefCell::new(curr)));
    }
    pub fn from_list(arr: &[Option<i32>]) -> Option<Rc<RefCell<Self>>> {
        Self::_from_list(arr, 0)
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn f(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        // (result taking current node, result not taking current node)
        if node.is_none() {
            return (0, 0);
        }

        let (left_taking, left_not_taking) = Self::f(&node.as_ref().unwrap().borrow().left);
        let (right_taking, right_not_taking) = Self::f(&node.as_ref().unwrap().borrow().right);

        let res_taking = node.as_ref().unwrap().borrow().val + left_not_taking + right_not_taking;
        let res_not_taking = left_taking.max(left_not_taking) + right_taking.max(right_not_taking);
        // let res_not_taking = left_taking + right_taking;

        (res_taking, res_not_taking)
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (res_taking, res_not_taking) = Self::f(&root);
        res_taking.max(res_not_taking)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // root = [3,2,3,null,3,null,1]
        let arr = [Some(3), Some(2), Some(3), None, Some(3), None, Some(1)];
        let root = TreeNode::from_list(arr.as_slice());
        let res = 7;
        assert_eq!(Solution::rob(root), res);
    }

    #[test]
    fn test2() {
        // root = [3,4,5,1,3,null,1]
        let arr = [Some(3), Some(4), Some(5), Some(1), Some(3), None, Some(1)];
        let root = TreeNode::from_list(arr.as_slice());
        let res = 9;
        assert_eq!(Solution::rob(root), res);
    }
}
