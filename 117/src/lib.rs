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
    fn dfs(node: Rc<RefCell<TreeNode>>, sum_so_far: i32) -> i32 {
        let new_sum = 10 * sum_so_far + node.borrow().val;
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            new_sum
        } else {
            let mut res = 0;
            if node.borrow().left.is_some() {
                res += Self::dfs(node.borrow().left.as_ref().unwrap().clone(), new_sum);
            }
            if node.borrow().right.is_some() {
                res += Self::dfs(node.borrow().right.as_ref().unwrap().clone(), new_sum);
            }
            res
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        Self::dfs(root.unwrap(), 0)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    fn create_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let root = create_node(vals[0].unwrap());
        let mut nodes = VecDeque::from([root.clone()]);
        let mut val_iter = vals.iter().skip(1);

        loop {
            let left_opt = val_iter.next();
            if left_opt.is_none() {
                break;
            }
            let node = nodes.pop_front().unwrap();
            let left_val = left_opt.unwrap();

            if left_val.is_some() {
                node.borrow_mut()
                    .left
                    .replace(create_node(left_val.unwrap()));
                let x = node.borrow_mut().left.as_mut().unwrap().clone();
                nodes.push_back(x);
            }

            let right_opt = val_iter.next();
            if right_opt.is_none() {
                break;
            }
            let right_val = right_opt.unwrap();

            if right_val.is_some() {
                node.borrow_mut()
                    .right
                    .replace(create_node(right_val.unwrap()));
                let x = node.borrow_mut().right.as_mut().unwrap().clone();
                nodes.push_back(x);
            }
        }

        Some(root)
    }
    fn parse_vals(val_str: &str) -> Vec<Option<i32>> {
        val_str
            .split(",")
            .map(|x| {
                if x == "null" {
                    None
                } else {
                    Some(x.parse::<i32>().unwrap())
                }
            })
            .collect()
    }

    #[test]
    fn test1() {
        let val_str = "1,2,3";
        let res = 25;

        let vals = parse_vals(val_str);
        let root = create_tree(&vals);

        assert_eq!(Solution::sum_numbers(root), res);
    }

    #[test]
    fn test2() {
        let val_str = "4,9,0,5,1";
        let res = 1026;

        let vals = parse_vals(val_str);
        let root = create_tree(&vals);

        assert_eq!(Solution::sum_numbers(root), res);
    }

    #[test]
    fn test_empty() {
        let root = None;
        let res = 0;

        assert_eq!(Solution::sum_numbers(root), res);
    }
}
