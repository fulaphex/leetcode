impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        const MODULUS: usize = 1_000_000_007;
        if complexity.iter().skip(1).min().unwrap() > &complexity[0] {
            (1..complexity.len()).fold(1, |acc, el| (acc as usize * el) % MODULUS) as i32
        } else {
            0
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let complexity = vec![1, 2, 3];
        let res = 2;
        assert_eq!(Solution::count_permutations(complexity), res);
    }

    #[test]
    fn test2() {
        let complexity = vec![3, 3, 3, 4, 4, 4];
        let res = 0;
        assert_eq!(Solution::count_permutations(complexity), res);
    }

    #[test]
    fn test3() {
        let complexity = vec![58, 283, 52];
        let res = 0;
        assert_eq!(Solution::count_permutations(complexity), res);
    }
}
