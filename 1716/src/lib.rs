impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut res = 0;
        for day_of_week in 0..=6 {
            let day_count = (n - day_of_week + 6) / 7;
            res += (day_count + 1) * day_count / 2 + day_count * day_of_week;
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
        let n = 4;
        let res = 10;
        assert_eq!(Solution::total_money(n), res);
    }

    #[test]
    fn test2() {
        let n = 10;
        let res = 37;
        assert_eq!(Solution::total_money(n), res);
    }

    #[test]
    fn test3() {
        let n = 20;
        let res = 96;
        assert_eq!(Solution::total_money(n), res);
    }
}
