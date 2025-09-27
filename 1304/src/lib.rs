impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        return (1..=(n / 2))
            .chain((-n / 2)..0)
            .chain(if n % 2 == 1 { vec![0] } else { vec![] })
            .collect();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        assert_eq!(Solution::sum_zero(n), [1, 2, -2, -1, 0]);
    }

    #[test]
    fn test2() {
        let n = 4;
        assert_eq!(Solution::sum_zero(n), [1, 2, -2, -1]);
    }

    #[test]
    fn test3() {
        let n = 1;
        assert_eq!(Solution::sum_zero(n), [0]);
    }
}
