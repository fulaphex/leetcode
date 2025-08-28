use std::collections;

struct Solution {}

impl Solution {
    fn perms(nums: &mut collections::HashMap<i32, i32>, acc: &mut Vec<i32>) -> i32 {
        if acc.len() > 1 {
            let last_sum = acc[acc.len() - 2] + acc[acc.len() - 1];
            let root = (last_sum as f64).sqrt();
            if (root as i32).pow(2) != last_sum {
                return 0;
            }
            if nums.len() == 0 {
                return 1;
            }
        }

        let mut res = 0;
        let keys = Vec::from_iter(nums.keys().cloned());
        for k in keys {
            let val = *nums.get(&k).unwrap();
            acc.push(k);

            if val == 1 {
                nums.remove(&k);
            } else {
                nums.insert(k, val - 1);
            }

            res += Self::perms(nums, acc);

            acc.pop();
            nums.insert(k, val);
        }
        return res;
    }

    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut el_counts: collections::HashMap<i32, i32> = collections::HashMap::new();
        for x in nums {
            el_counts.insert(x, el_counts.get(&x).unwrap_or(&0) + 1);
        }
        return Self::perms(&mut el_counts, &mut vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 17, 8];
        let res = 2;
        assert_eq!(Solution::num_squareful_perms(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 2];
        let res = 1;
        assert_eq!(Solution::num_squareful_perms(nums), res);
    }
}
