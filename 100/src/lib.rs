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
    fn is_same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_some() ^ q.is_some() {
            return false;
        }
        if p.is_none() && q.is_none() {
            return true;
        }
        return (p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val)
            && Self::is_same(
                &p.as_ref().unwrap().borrow().left,
                &q.as_ref().unwrap().borrow().left,
            )
            && Self::is_same(
                &p.as_ref().unwrap().borrow().right,
                &q.as_ref().unwrap().borrow().right,
            );
    }
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::is_same(&p, &q)
    }
}

struct Solution {}
