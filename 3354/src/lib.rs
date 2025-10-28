impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let total = nums.iter().sum::<i32>();

        for start_idx in 0..nums.len() {
            if nums[start_idx] != 0 {
                continue;
            }
            for mut dx in [-1, 1] {
                let mut idx = start_idx as i32;
                let mut curr_nums = nums.clone();
                let mut removed = 0;

                while removed < total {
                    // check boundary
                    if idx >= nums.len() as i32 || idx < 0 {
                        break;
                    }

                    // remove the number
                    if curr_nums[idx as usize] > 0 {
                        curr_nums[idx as usize] -= 1;
                        dx *= -1;
                        removed += 1;
                    }

                    // advance the step
                    idx += dx;
                }

                if removed == total {
                    res += 1;
                }
            }
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
        let nums = vec![1, 0, 2, 0, 3];
        let res = 2;
        assert_eq!(Solution::count_valid_selections(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 3, 4, 0, 4, 1, 0];
        let res = 0;
        assert_eq!(Solution::count_valid_selections(nums), res);
    }
}
