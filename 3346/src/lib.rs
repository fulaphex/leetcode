use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort();
        let num_operations = num_operations as usize;
        let mut freq = HashMap::new();
        for x in &nums {
            *freq.entry(x).or_insert(0) += 1;
        }
        let max_freq = freq.values().max().unwrap();

        let mut res = 1;

        let mut que = nums.iter().peekable();
        let mut curr = VecDeque::new();
        for x in &nums {
            // add elements `el` from que that `el <= x + 2k`
            loop {
                let next_el_opt = que.peek();
                if next_el_opt.is_none() {
                    break;
                }
                let el = **next_el_opt.unwrap();
                if el <= x.saturating_add(k).saturating_add(k) {
                    que.next();
                    curr.push_back(el);
                } else {
                    break;
                }
            }

            // remove elements `el` from que that `el < x`
            loop {
                let first_el = curr.front().unwrap();
                if first_el < x {
                    curr.pop_front();
                } else {
                    break;
                }
            }

            res = res.max(curr.len().min(num_operations));
            if res == num_operations {
                break;
            }
        }

        let mut curr = VecDeque::new();
        let mut que = nums.iter().peekable();
        for x in &nums {
            // add elements `el` from que that `el <= x + k`
            loop {
                let next_el_opt = que.peek();
                if next_el_opt.is_none() {
                    break;
                }
                let el = **next_el_opt.unwrap();
                if el <= x.saturating_add(k) {
                    que.next();
                    curr.push_back(el);
                } else {
                    break;
                }
            }

            // remove elements `el` from que that `el < x - k`
            loop {
                let first_el = *curr.front().unwrap();
                if first_el < x.saturating_sub(k) {
                    curr.pop_front();
                } else {
                    break;
                }
            }

            res = res.max(curr.len().min(num_operations + freq.get(x).unwrap()));
            if res == num_operations + max_freq {
                break;
            }
        }

        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 4, 5];
        let k = 1;
        let num_operations = 2;
        let res = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), res);
    }

    #[test]
    fn test2() {
        let nums = vec![999999997, 999999999, 999999999];
        let k = 999999999;
        let num_operations = 2;
        let res = 3;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), res);
    }
}
