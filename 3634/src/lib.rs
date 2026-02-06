impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut res = 0;
        let (mut start_idx, mut end_idx) = (0, 0);

        while start_idx < nums.len() {
            loop {
                if end_idx + 1 == nums.len() {
                    break;
                }
                if nums[start_idx] * k >= nums[end_idx + 1] {
                    end_idx += 1;
                } else {
                    break;
                }
            }

            res = res.max(end_idx - start_idx + 1);
            start_idx += 1;
        }

        (nums.len() - res) as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 1, 5];
        let k = 2;
        let res = 1;

        assert_eq!(Solution::min_removal(nums, k), res);
    }
}
