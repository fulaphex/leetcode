use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut num_strs: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();

        fn cmp(a: &String, b: &String) -> Ordering {
            let first_iter = a.chars().chain(b.chars());
            let second_iter = b.chars().chain(a.chars());
            return second_iter.cmp(first_iter);
        }
        num_strs.sort_by(cmp);

        let res = num_strs.join("");
        if res.chars().all(|x| x == '0') {
            return String::from("0");
        }
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![10, 2];
        let res = String::from("210");
        assert_eq!(Solution::largest_number(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 30, 34, 5, 9];
        let res = String::from("9534330");
        assert_eq!(Solution::largest_number(nums), res);
    }
}
