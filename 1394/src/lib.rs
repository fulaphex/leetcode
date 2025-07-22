use std::collections;

struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counts = collections::HashMap::<i32, i32>::new();
        for x in &arr {
            let val = counts.get(x).unwrap_or(&0) + 1;
            counts.insert(*x, val);
        }
        let mut res = -1;
        for (k, v) in counts {
            if k == v {
                res = res.max(v);
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![2, 2, 3, 4];
        let res = 2;
        assert_eq!(Solution::find_lucky(arr), res);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        let res = 3;
        assert_eq!(Solution::find_lucky(arr), res);
    }

    #[test]
    fn test3() {
        let arr = vec![2, 2, 2, 3, 3];
        let res = -1;
        assert_eq!(Solution::find_lucky(arr), res);
    }
}
