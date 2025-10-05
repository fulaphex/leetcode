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
        let mut res = vec![self.val];
        let mut node = &self.next;
        while node.is_some() {
            res.push(node.as_ref().unwrap().val);
            node = &node.as_ref().unwrap().next;
        }
        return res;
    }
}

struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // adding an extra node at the beginning because first elements might have to be removed
        let mut prehead = Some(Box::new(ListNode::new(0)));
        let _ = std::mem::replace(&mut prehead.as_mut().unwrap().next, head);

        // current working element
        let mut curr = &mut prehead;
        loop {
            if curr.is_none() {
                break;
            }
            let next_node_option = &curr.as_ref().unwrap().next;
            if next_node_option.is_none() {
                break;
            }
            let nextval = next_node_option.as_ref().unwrap().val;

            let next_node_option2 = &next_node_option.as_ref().unwrap().next;
            if next_node_option2.is_none() {
                break;
            }
            let nextval2 = next_node_option2.as_ref().unwrap().val;

            if nextval != nextval2 {
                // next two nodes don't share the value
                curr = &mut curr.as_mut().unwrap().next;
            } else {
                // next two nodes share the same value

                // unlink the next node
                let next_node_option = std::mem::replace(&mut curr.as_mut().unwrap().next, None);

                // find a node (or None) that has a different value than nextval
                let mut diff_node = next_node_option;
                loop {
                    if diff_node.is_none() {
                        break;
                    }
                    if diff_node.as_ref().unwrap().val != nextval {
                        break;
                    }
                    diff_node = diff_node.unwrap().next;
                }

                // insert that value as the successor of curr
                let _ = std::mem::replace(&mut curr.as_mut().unwrap().next, diff_node);

                // cannot advance here because the next segment might need to be removed as well
            }
        }

        // skipping the first extra node
        return prehead.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        for &x in arr.iter() {
            let node = ListNode::new(x);
            let _ = std::mem::replace(&mut tail.as_mut().unwrap().next, Some(Box::new(node)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        return head.unwrap().next;
    }

    #[test]
    fn test1() {
        let arr = vec![1, 2, 3, 3, 4, 4, 5];
        let list = create_list(&arr);
        let res_node = Solution::delete_duplicates(list);
        let res_vec = res_node.unwrap().into_vec();
        let res = vec![1, 2, 5];
        assert_eq!(res_vec, res);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 1, 1, 2, 3];
        let list = create_list(&arr);
        let res_node = Solution::delete_duplicates(list);
        let res_vec = res_node.unwrap().into_vec();
        let res = vec![2, 3];
        assert_eq!(res_vec, res);
    }

    #[test]
    fn test3() {
        let arr = vec![1, 1, 1, 2, 2];
        let list = create_list(&arr);
        let res_node = Solution::delete_duplicates(list);
        let res_vec = res_node.unwrap().into_vec();
        let res = vec![2, 3];
        assert_eq!(res_vec, res);
    }
}
