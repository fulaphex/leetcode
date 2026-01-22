impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;

        loop {
            let mut sorted = true;
            let mut min_pair = (i32::MAX, 0);

            for (idx, w) in nums.windows(2).enumerate() {
                if w[0] > w[1] {
                    sorted = false;
                }
                if w[0] + w[1] < min_pair.0 {
                    min_pair = (w[0] + w[1], idx);
                }
            }

            if sorted {
                return res;
            }

            res += 1;

            nums[min_pair.1] = min_pair.0;
            for i in min_pair.1 + 2..nums.len() {
                nums[i - 1] = nums[i];
            }
            nums.pop();
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 2, 3, 1];
        let res = 2;

        assert_eq!(Solution::minimum_pair_removal(nums), res);
    }
}
