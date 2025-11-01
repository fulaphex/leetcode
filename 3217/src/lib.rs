// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let to_remove: HashSet<i32> = nums.into_iter().collect();

        let mut prehead = Some(Box::new(ListNode::new(0)));
        prehead.as_mut().unwrap().next.replace(head.unwrap());

        let mut curr = prehead.as_mut().unwrap().as_mut();

        while curr.next.is_some() {
            let next_val = curr.next.as_ref().unwrap().val;
            if to_remove.contains(&next_val) {
                let next_next = curr.next.as_mut().unwrap().next.take();
                let _ = std::mem::replace(&mut curr.next, next_next);
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        prehead.unwrap().next
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut prehead = ListNode::new(0);
        let mut curr = &mut prehead;
        for x in arr {
            curr.next.replace(Box::new(ListNode::new(x)));
            curr = curr.next.as_mut().unwrap().as_mut();
        }
        prehead.next
    }

    fn linked_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut curr = head;

        while curr.is_some() {
            res.push(curr.as_ref().unwrap().val);
            curr = &curr.as_ref().unwrap().next;
        }

        res
    }

    #[test]
    fn test1() {
        let arr = vec![1, 2, 3, 4, 5];
        let nums = vec![1, 2, 3];
        let head = create_list(arr);
        let res = vec![4, 5];

        let out = Solution::modified_list(nums, head);
        let out_vec = linked_to_vec(&out);

        assert_eq!(out_vec, res);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 2, 1, 2, 1, 2];
        let nums = vec![1];
        let head = create_list(arr);
        let res = vec![2, 2, 2];

        let out = Solution::modified_list(nums, head);
        let out_vec = linked_to_vec(&out);

        assert_eq!(out_vec, res);
    }

    #[test]
    fn test3() {
        let arr = vec![1, 2, 3, 4];
        let nums = vec![5];
        let head = create_list(arr);
        let res = vec![1, 2, 3, 4];

        let out = Solution::modified_list(nums, head);
        let out_vec = linked_to_vec(&out);

        assert_eq!(out_vec, res);
    }
}
