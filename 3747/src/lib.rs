impl Solution {
    pub fn count_distinct(n: i64) -> i64 {
        match n <= 10 {
            true => n.min(9),
            _ => {
                let log10 = n.ilog10();
                Self::count_distinct((n - 10_i64.pow(log10)).max(10_i64.pow(log10 - 1)))
                    + 9_i64.pow(log10)
            }
        }
    }
}

struct Solution {}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 9)]
    #[case(23, 21)]
    #[case(123, 102)]
    #[case(100, 90)]
    #[case(101, 90)]
    fn test(#[case] n: i64, #[case] res: i64) {
        assert_eq!(Solution::count_distinct(n), res);
    }
}
