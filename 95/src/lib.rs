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
    fn generate(els: &[i32]) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if els.is_empty() {
            return vec![None];
        }
        let mut res = vec![];

        for (idx, &el) in els.iter().enumerate() {
            let (left_elements, right_elements) = (&els[..idx], &els[idx + 1..]);

            let left_subtrees = Self::generate(left_elements);
            let right_subtrees = Self::generate(right_elements);

            for l in &left_subtrees {
                for r in &right_subtrees {
                    let mut node = TreeNode::new(el);
                    if l.is_some() {
                        node.left.replace(l.as_ref().unwrap().clone());
                    }
                    if r.is_some() {
                        node.right.replace(r.as_ref().unwrap().clone());
                    }
                    res.push(Some(Rc::new(RefCell::new(node))));
                }
            }
        }
        res
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let elements: Vec<i32> = (1..=n).collect();
        Self::generate(&elements)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let out = Solution::generate_trees(n);

        assert_eq!(out.len(), 5);
    }
}
