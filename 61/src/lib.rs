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

    fn into_vec(self) -> Vec<i32> {
        let mut res = vec![];
        let mut curr = &self;
        while curr.next.is_some() {
            res.push(curr.val);
            curr = curr.next.as_ref().unwrap();
        }
        res.push(curr.val);
        res
    }
}

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        // figure out the length of the list
        let mut len = 0;
        let mut curr = &head;
        while curr.is_some() {
            len += 1;
            curr = &curr.as_ref().unwrap().next;
        }

        // figure out how many elements remove from the front
        let shift = (len - (k % len)) % len;

        // if we don't have to shift - return early
        if shift == 0 {
            return head;
        }

        let mut pre_new_head = &mut head;
        for _ in 0..(shift - 1) {
            pre_new_head = &mut pre_new_head.as_mut().unwrap().next;
        }
        let mut new_head = pre_new_head.as_mut().unwrap().next.take();
        let mut last_element = &mut new_head;
        while last_element.as_ref().unwrap().next.is_some() {
            last_element = &mut last_element.as_mut().unwrap().next;
        }
        last_element.as_mut().unwrap().next.replace(head.unwrap());

        new_head
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut prehead = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut prehead;
        for &v in vals {
            curr.as_mut()
                .unwrap()
                .next
                .replace(Box::new(ListNode::new(v)));
            curr = &mut curr.as_mut().unwrap().next;
        }
        prehead.unwrap().next
    }

    #[test]
    fn test() {
        let vals = [1, 2, 3, 4, 5];
        let head = create_list(&vals);
        let k = 2;
        let res = vec![4, 5, 1, 2, 3];

        let out = Solution::rotate_right(head, k);
        assert_eq!(out.unwrap().into_vec(), res);
    }

    #[test]
    fn test_empty() {
        let vals = [];
        let head = create_list(&vals);
        let k = 2;

        let out = Solution::rotate_right(head, k);
        assert!(out.is_none());
    }
}
