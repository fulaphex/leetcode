use std::collections;

struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut counts = collections::HashMap::<i32, i32>::new();
        let (mut left, mut right) = (0, 0);
        let mut res = 0;

        while right < fruits.len() {
            let fruit = fruits[right];
            counts.insert(fruit, counts.get(&fruit).unwrap_or(&0) + 1);

            while counts.len() > 2 {
                let left_fruit = fruits[left];
                if *counts.get(&left_fruit).unwrap() == 1 {
                    counts.remove(&left_fruit);
                } else {
                    *counts.get_mut(&left_fruit).unwrap() -= 1;
                }
                left += 1;
            }

            res = res.max(right + 1 - left);
            right += 1;
        }

        return res as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let fruits = vec![1, 2, 1];
        let res = 3;
        assert_eq!(Solution::total_fruit(fruits), res);
    }

    #[test]
    fn test2() {
        let fruits = vec![0, 1, 2, 2];
        let res = 3;
        assert_eq!(Solution::total_fruit(fruits), res);
    }

    #[test]
    fn test3() {
        let fruits = vec![1, 2, 3, 2, 2];
        let res = 4;
        assert_eq!(Solution::total_fruit(fruits), res);
    }
}
