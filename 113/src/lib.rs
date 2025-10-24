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
    fn dfs(
        node_opt: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        mut sum_so_far: i32,
        acc: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if node_opt.is_none() {
            if sum_so_far == target && acc.len() > 0 {
                res.push(acc.clone());
            }
            return;
        }

        let node = node_opt.as_ref().unwrap().borrow();
        sum_so_far += node.val;
        acc.push(node.val);
        if node.left.is_none() && node.right.is_none() {
            if sum_so_far == target {
                res.push(acc.clone());
            }
        } else {
            if node.left.is_some() {
                Self::dfs(node.left.clone(), target, sum_so_far, acc, res)
            }
            if node.right.is_some() {
                Self::dfs(node.right.clone(), target, sum_so_far, acc, res)
            }
        }
        acc.pop();
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::dfs(root, target_sum, 0, &mut vec![], &mut res);
        res
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
        let val_str = "5,4,8,11,null,13,4,7,2,null,null,5,1";
        let target_sum = 22;
        let res: Vec<Vec<i32>> = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];

        let vals = parse_vals(val_str);
        let root = create_tree(&vals);
        assert_eq!(Solution::path_sum(root, target_sum), res);
    }

    #[test]
    fn test2() {
        let val_str = "1,2,3";
        let target_sum = 5;
        let res: Vec<Vec<i32>> = vec![];

        let vals = parse_vals(val_str);
        let root = create_tree(&vals);
        assert_eq!(Solution::path_sum(root, target_sum), res);
    }

    #[test]
    fn test3() {
        let val_str = "1,2";
        let target_sum = 0;
        let res: Vec<Vec<i32>> = vec![];

        let vals = parse_vals(val_str);
        let root = create_tree(&vals);
        assert_eq!(Solution::path_sum(root, target_sum), res);
    }

    #[test]
    fn test_empty() {
        let root = None;
        let target_sum = 1;
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root.clone(), target_sum), res);

        let target_sum = 0;
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root, target_sum), res);
    }
}
