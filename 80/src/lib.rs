struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;
        let mut el = nums[0];
        let mut dups = 0;
        for idx in 1..nums.len() {
            println!("nums[idx]: {}", nums[idx]);
            if nums[idx] == el {
                count += 1;
                if count > 2 {
                    dups += 1;
                }
            } else {
                el = nums[idx];
                count = 1;
            }
            println!("el: {}; dups: {}; count: {}", el, dups, count);
            if count < 3 {
                nums[idx - dups] = nums[idx];
            }
        }
        return (nums.len() - dups) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(nums[..7], vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
