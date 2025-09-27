impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;

        for (idx1, a) in nums.iter().enumerate() {
            let mut idx3 = 0;
            for idx2 in idx1 + 1..nums.len() {
                idx3 = idx3.max(idx2 + 1);
                let b = nums[idx2];

                while idx3 < nums.len() && nums[idx3] < (a + b) {
                    idx3 += 1;
                }

                res += (idx3 - (idx2 + 1)) as i32;
            }
        }

        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 2, 3, 4];
        let res = 3;
        assert_eq!(Solution::triangle_number(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 2, 3, 4];
        let res = 4;
        assert_eq!(Solution::triangle_number(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![0, 0];
        let res = 0;
        assert_eq!(Solution::triangle_number(nums), res);
    }
}
