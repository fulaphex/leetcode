struct Solution {}

impl Solution {
    pub fn weird_binsearch(nums: &[i32], target: i32) -> bool {
        println!("calling weird binsrch on {:?} with {}", nums, target);
        if nums[0] < nums[nums.len() - 1] {
            return Self::binsearch(nums, target);
        }
        if nums.len() <= 2 {
            for &x in nums {
                if x == target {
                    return true;
                }
            }
            return false;
        }
        let mid_idx = nums.len() / 2;
        let mid = nums[mid_idx];
        println!("mid: {}", mid);
        if target == mid {
            return true;
        }
        if nums[0] == mid && mid == nums[nums.len() - 1] {
            return Self::weird_binsearch(&nums[..mid_idx], target)
                || Self::weird_binsearch(&nums[mid_idx..], target);
        } else {
            if mid >= nums[0] {
                if target < mid && target >= nums[0] {
                    return Self::weird_binsearch(&nums[..mid_idx], target);
                } else {
                    return Self::weird_binsearch(&nums[mid_idx..], target);
                }
            } else {
                // mid <= nums[-1];
                if mid < target && target <= nums[nums.len() - 1] {
                    return Self::weird_binsearch(&nums[mid_idx..], target);
                } else {
                    return Self::weird_binsearch(&nums[..mid_idx], target);
                }
            }
        }
    }
    pub fn binsearch(nums: &[i32], target: i32) -> bool {
        println!("calling binsrch on {:?} with {}", nums, target);
        if nums.len() <= 2 {
            for &x in nums {
                if x == target {
                    return true;
                }
            }
            return false;
        }
        let mid_idx = nums.len() / 2;
        let mid = nums[mid_idx];
        if target == mid {
            return true;
        }
        if target > mid {
            return Self::binsearch(&nums[mid_idx..], target);
        } else {
            return Self::binsearch(&nums[..mid_idx], target);
        }
    }
    pub fn search(nums: &Vec<i32>, target: i32) -> bool {
        println!("searching for {} in {:?}", target, nums);
        let res = Self::weird_binsearch(nums, target);
        println!();
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        assert_eq!(Solution::search(&nums, 2), true);
        assert_eq!(Solution::search(&nums, 5), true);
        assert_eq!(Solution::search(&nums, 6), true);
        assert_eq!(Solution::search(&nums, 0), true);
        assert_eq!(Solution::search(&nums, 1), true);
        assert_eq!(Solution::search(&nums, 3), false);
        assert_eq!(Solution::search(&nums, 4), false);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 0, 1, 1, 1];
        assert_eq!(Solution::search(&nums, 1), true);
        assert_eq!(Solution::search(&nums, 0), true);
    }
}
