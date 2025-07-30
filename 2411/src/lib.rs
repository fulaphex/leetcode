struct Solution {}

impl Solution {
    #[inline]
    pub fn add(acc: &mut Vec<i16>, mut el: i32) {
        for idx in 0..acc.len() {
            acc[idx] += (el % 2) as i16;
            el /= 2;
        }
    }
    #[inline]
    pub fn remove(acc: &mut Vec<i16>, mut el: i32) {
        for idx in 0..acc.len() {
            acc[idx] -= (el % 2) as i16;
            el /= 2;
        }
    }
    #[inline]
    pub fn check(acc: &Vec<i16>, mut el: i32) -> bool {
        for idx in 0..acc.len() {
            if el % 2 == 1 {
                if acc[idx] == 0 {
                    return false;
                }
            }
            el /= 2;
        }
        return true;
    }
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![nums.len() as i32 + 2; nums.len()];
        let start_iter = nums.iter().enumerate().rev();
        let mut end_iter = nums.iter().rev().peekable();
        let mut acc = vec![0 as i16; 30];
        let mut size = 0;
        let mut or_val = 0;

        for (idx, el) in start_iter {
            or_val |= *el;
            Self::add(&mut acc, *el);
            size += 1;

            // try removing end element
            while Self::check(&acc, or_val) && size > 0 {
                let end_el = end_iter.next().unwrap();
                Self::remove(&mut acc, *end_el);
                size -= 1;
            }
            res[idx as usize] = res[idx as usize].min(size + 1);
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 0, 2, 1, 3];
        let res = vec![3, 3, 2, 2, 1];
        assert_eq!(Solution::smallest_subarrays(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2];
        let res = vec![2, 1];
        assert_eq!(Solution::smallest_subarrays(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![0];
        let res = vec![1];
        assert_eq!(Solution::smallest_subarrays(nums), res);
    }
}
