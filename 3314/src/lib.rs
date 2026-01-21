impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for x in nums {
            let mut curr = -1;
            for i in 0..x {
                if i | (i + 1) == x {
                    curr = i;
                    break;
                }
            }
            res.push(curr);
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
        let nums = vec![2, 3, 5, 7];
        let res = vec![-1, 1, 4, 3];
        assert_eq!(Solution::min_bitwise_array(nums), res);
    }
}
