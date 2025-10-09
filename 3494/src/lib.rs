impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut earliest_free = vec![0; skill.len() + 1];

        for m in mana {
            for (mage_idx, &s) in skill.iter().enumerate() {
                earliest_free[mage_idx + 1] =
                    earliest_free[mage_idx + 1].max(earliest_free[mage_idx]) + (s * m) as i64;
            }

            for (mage_idx, &s) in skill.iter().enumerate().rev() {
                earliest_free[mage_idx] = earliest_free[mage_idx + 1] - (s * m) as i64;
            }
        }
        return earliest_free[earliest_free.len() - 1];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let skill = vec![1, 5, 2, 4];
        let mana = vec![5, 1, 4, 2];
        let res = 110;
        assert_eq!(Solution::min_time(skill, mana), res);
    }

    #[test]
    fn test2() {
        let skill = vec![1; 3];
        let mana = vec![1; 3];
        let res = 5;
        assert_eq!(Solution::min_time(skill, mana), res);
    }

    #[test]
    fn test3() {
        let skill = vec![1, 2, 3, 4];
        let mana = vec![1, 2];
        let res = 2221;
        assert_eq!(Solution::min_time(skill, mana), res);
    }
}
