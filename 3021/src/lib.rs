impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        // (first is even, second is odd) + (first is odd, second is even)
        return (n / 2) as i64 * ((m + 1) / 2) as i64 + ((n + 1) / 2) as i64 * (m / 2) as i64;
    }
}
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (n, m) = (2, 3);
        let res = 3;
        assert_eq!(Solution::flower_game(n, m), res);
    }

    #[test]
    fn test2() {
        let (n, m) = (1, 1);
        let res = 0;
        assert_eq!(Solution::flower_game(n, m), res);
    }
}
