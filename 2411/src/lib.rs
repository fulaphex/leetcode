struct Solution {}

impl Solution {
    pub fn add(acc: &mut Vec<i32>, mut el: i32) {
        for idx in 0..acc.len() {
            acc[idx] += el % 2;
            el /= 2;
        }
    }
    pub fn remove(acc: &mut Vec<i32>, mut el: i32) {
        for idx in 0..acc.len() {
            acc[idx] -= el % 2;
            el /= 2;
        }
    }
    pub fn check(acc: &Vec<i32>, mut el: i32) -> bool {
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
        let (mut start_iter, mut end_iter) = (nums.iter().rev(), nums.iter().rev().peekable());
        let mut acc = vec![0; 22];
        let mut size = 0;
        let mut or_val = 0;
        let mut idx = res.len() as i32 - 1;

        let mut acc_vals: Vec<i32> = vec![];
        loop {
            let el_option = start_iter.next();
            if el_option.is_none() {
                break;
            }
            let el = el_option.unwrap();
            or_val |= *el;
            Self::add(&mut acc, *el);
            size += 1;
            res[idx as usize] = res[idx as usize].min(size);

            // try removing end element
            while size > 1 {
                let end_el = *end_iter.peek().unwrap();
                Self::remove(&mut acc, *end_el);
                if !Self::check(&acc, or_val) {
                    Self::add(&mut acc, *end_el);
                    break;
                } else {
                    size -= 1;
                    end_iter.next();
                }
            }
            res[idx as usize] = res[idx as usize].min(size);
            idx -= 1;
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
}
