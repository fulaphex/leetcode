use std::collections;

struct Solution {}

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let uk = k as usize;
        let mut buff = collections::VecDeque::new();

        for (pos, &x) in nums.iter().enumerate() {
            while buff.len() > 0 && *buff.get(0).unwrap() + uk <= pos {
                buff.pop_front();
            }
            let curr = (x + buff.len() as i32) % 2;

            if pos + uk > nums.len() {
                // println!("{:?} {:?}", (pos, x, curr), buff);
                if curr == 0 {
                    return -1;
                }
            } else {
                if curr == 0 {
                    res += 1;
                    buff.push_back(pos);
                }
                // while buff.len() > 0 && *buff.get(0).unwrap() + uk <= pos {
                //     buff.pop_front();
                // }
                // println!("{:?} {:?}", (pos, x, curr), buff);
            }
        }

        // println!("{:?}", buff);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![0, 1, 0];
        let k = 2;
        let res = 2;
        assert_eq!(Solution::min_k_bit_flips(nums, k), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 0];
        let k = 2;
        let res = -1;
        assert_eq!(Solution::min_k_bit_flips(nums, k), res);
    }

    #[test]
    fn test3() {
        let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
        let k = 3;
        let res = 3;
        assert_eq!(Solution::min_k_bit_flips(nums, k), res);
    }
}
