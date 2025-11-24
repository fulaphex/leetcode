impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        const MOD: i32 = 5;
        let (mut x, mut res) = (0, vec![]);
        for n in nums {
            x = (2 * x + n) % MOD;
            res.push(x == 0);
        }
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,1], vec![true,false,false])]
    #[case(vec![1,1,1], vec![false,false,false])]
    fn test(#[case] nums: Vec<i32>, #[case] res: Vec<bool>) {
        assert_eq!(Solution::prefixes_div_by5(nums), res);
    }
}
