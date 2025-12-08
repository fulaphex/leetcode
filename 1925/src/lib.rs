impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut res = 0;
        for a in 1..=n {
            for b in 1..a {
                if a * a + b * b > n * n {
                    break;
                }
                let c = (a * a + b * b).isqrt();
                if a * a + b * b == c * c {
                    res += 2;
                }
            }
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
        let n = 5;
        let res = 2;
        assert_eq!(Solution::count_triples(n), res);
    }
}
