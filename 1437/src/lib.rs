impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut dist = k + 1;
        for n in nums {
            if n == 0 {
                dist += 1;
            } else if dist < k {
                return false;
            } else {
                dist = 0;
            }
        }
        true
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 0, 0, 0, 1, 0, 0, 1];
        let k = 2;
        let res = true;

        assert_eq!(Solution::k_length_apart(nums, k), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 0, 0, 1, 0, 1];
        let k = 2;
        let res = false;

        assert_eq!(Solution::k_length_apart(nums, k), res);
    }
}
