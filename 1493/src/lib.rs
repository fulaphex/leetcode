struct Solution {}

impl Solution {
    pub fn longest_subarray(mut nums: Vec<i32>) -> i32 {
        let nulval = usize::MAX - 10;
        let mut res = 0;
        let (mut prev_zero, mut start_pos) = (nulval, nulval);

        for (pos, &x) in nums.iter().chain([&0, &0]).enumerate() {
            if x == 1 {
                if start_pos == nulval {
                    start_pos = pos;
                }
            }
            if x == 0 {
                if (start_pos == nulval) || (prev_zero == nulval) {
                    prev_zero = pos;
                } else if (prev_zero + 1) == pos {
                    // 111111111100
                    // two zeros in a row - (prev_zero == pos-1)
                    res = res.max(prev_zero - start_pos);
                    start_pos = nulval;
                    prev_zero = pos;
                } else {
                    // 111111111111110111111111111x
                    // start_pos
                    res = res.max(pos - start_pos - 1);
                    start_pos = prev_zero + 1;
                    prev_zero = pos;
                }
            }
        }
        return res.min(nums.len() - 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 0, 1];
        let res = 3;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let res = 5;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1];
        let res = 2;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test4() {
        let nums = vec![0; 3];
        let res = 0;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test5() {
        let nums = vec![
            0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1,
            1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1,
            1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
            0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1,
        ];
        let res = 60;

        assert_eq!(Solution::longest_subarray(nums), res);
    }
}
