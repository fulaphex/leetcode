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

struct Solution {}

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        let mut curr = head;
        while curr.is_some() {
            res += (*curr.clone().unwrap()).val;
            curr = (*curr.unwrap()).next;
        }
        return res;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let node = Option::Some(Box::new(ListNode::new(0)));
        assert_eq!(Solution::get_decimal_value(node), 0);
    }
}
