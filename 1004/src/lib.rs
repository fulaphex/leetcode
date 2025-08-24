use std::collections;

struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut buff = collections::VecDeque::new();
        buff.reserve(nums.len());
        let (mut res, mut curr, mut sum) = (0, 0, 0);
        for &n in nums.iter().chain([&0]) {
            if n == 1 {
                curr += 1;
            } else {
                sum += curr;
                buff.push_back(curr);
                curr = 0;

                if buff.len() > (k as usize + 1) {
                    let x = buff.pop_front().unwrap();
                    sum -= x;
                }
                println!("{:?} {}", buff, sum);

                res = res.max(sum + buff.len() - 1);
            }
        }
        return res as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;
        let res = 6;
        assert_eq!(Solution::longest_ones(nums, k), res);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;
        let res = 10;
        assert_eq!(Solution::longest_ones(nums, k), res);
    }

    #[test]
    fn test3() {
        let nums = vec![0; 4];
        let k = 3;
        let res = 3;
        assert_eq!(Solution::longest_ones(nums, k), res);
    }

    #[test]
    fn test4() {
        let nums = vec![1; 4];
        let k = 3;
        let res = 4;
        assert_eq!(Solution::longest_ones(nums, k), res);
    }
}
