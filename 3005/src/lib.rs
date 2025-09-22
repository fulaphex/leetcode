use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        let &max_freq = freq.values().max().unwrap();
        return freq.values().filter(|&&x| x == max_freq).count() as i32 * max_freq;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 2, 3, 1, 4];
        let res = 4;
        assert_eq!(Solution::max_frequency_elements(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4, 5];
        let res = 5;
        assert_eq!(Solution::max_frequency_elements(nums), res);
    }
}
