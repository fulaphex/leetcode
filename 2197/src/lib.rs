use std::collections::VecDeque;

impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        return a;
    }
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut res = VecDeque::new();
        for &x in nums.iter() {
            let mut curr = x;
            loop {
                let last_op = res.back();
                if last_op.is_none() {
                    res.push_back(curr);
                    break;
                }
                let last = *last_op.unwrap();
                let gcd = Self::gcd(curr, last);

                if gcd == 1 {
                    res.push_back(curr);
                    break;
                } else {
                    res.pop_back();
                    curr = (last / gcd) * curr;
                }
            }
        }

        return res.into_iter().collect::<Vec<_>>();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![6, 4, 3, 2, 7, 6, 2];
        let res = vec![12, 7, 6];
        assert_eq!(Solution::replace_non_coprimes(nums), res);
    }
}
