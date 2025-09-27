struct Solution {}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut counts = vec![0; 32];
        for mut c in candidates {
            let mut idx = 0;
            while c > 0 {
                if c % 2 == 1 {
                    counts[idx] += 1;
                }
                idx += 1;
                c /= 2;
            }
        }
        return counts.iter().fold(0, |x, &y| x.max(y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![16, 17, 71, 62, 12, 24, 14];
        let res = 4;
        assert_eq!(Solution::largest_combination(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![8, 8];
        let res = 2;
        assert_eq!(Solution::largest_combination(nums), res);
    }
}
