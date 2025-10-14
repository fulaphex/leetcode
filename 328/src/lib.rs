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

    #[inline]
    fn into_vec(&self) -> Vec<i32> {
        let mut res = vec![self.val];
        let mut curr = self;
        while curr.next.is_some() {
            res.push(curr.next.as_ref().unwrap().val);
            curr = &curr.next.as_ref().unwrap();
        }
        return res;
    }
}

fn create_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut prehead = Some(Box::new(ListNode::new(0)));
    let mut curr_node = &mut prehead;
    for &v in vals {
        let node = Some(Box::new(ListNode::new(v)));
        let _ = std::mem::replace(&mut curr_node.as_mut().unwrap().next, node);
        curr_node = &mut curr_node.as_mut().unwrap().next;
    }
    return prehead.unwrap().next;
}

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut even_prehead = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut head;
        let mut curr_even = &mut even_prehead;
        loop {
            // moving out the next element
            if curr.is_none() {
                break;
            }
            let mut next_node = std::mem::replace(&mut curr.as_mut().unwrap().next, None);
            if next_node.is_none() {
                // nothing more to do
                break;
            }
            // moving out the second next element
            let mut next_node2 = std::mem::replace(&mut next_node.as_mut().unwrap().next, None);

            // placing the second next as next
            let _ = std::mem::replace(&mut curr.as_mut().unwrap().next, next_node2);

            // placing the next element on the even element list
            let _ = std::mem::replace(&mut curr_even.as_mut().unwrap().next, next_node);

            // advancing the current element
            curr = &mut curr.as_mut().unwrap().next;
            curr_even = &mut curr_even.as_mut().unwrap().next;
        }

        let mut last_el = &mut head;
        while last_el.as_ref().unwrap().next.is_some() {
            last_el = &mut last_el.as_mut().unwrap().next;
        }

        let _ = std::mem::replace(
            &mut last_el.as_mut().unwrap().next,
            even_prehead.unwrap().next,
        );

        head
    }
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut curr = head;
    while curr.is_some() {
        println!("node: {}", curr.as_ref().unwrap().val);
        curr = &curr.as_ref().unwrap().next;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_odd_even(arr: Vec<i32>, res: Vec<i32>) {
        let head = create_list(arr.as_slice());

        let out = Solution::odd_even_list(head);
        let out_vec = if out.is_some() {
            out.unwrap().into_vec()
        } else {
            vec![]
        };
        assert_eq!(res, out_vec);
    }

    #[test]
    fn test1() {
        let arr = vec![1, 2, 3, 4, 5];
        let res = vec![1, 3, 5, 2, 4];

        test_odd_even(arr, res);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let res = vec![1, 3, 5, 2, 4, 6];

        test_odd_even(arr, res);
    }

    #[test]
    fn test3() {
        let arr = vec![];
        let res = vec![];

        test_odd_even(arr, res);
    }
}
