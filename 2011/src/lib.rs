impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        (operations.len()
            - 2 * operations
                .iter()
                .filter(|&x| x.as_bytes()[1] == b'-')
                .count()) as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let operations: Vec<_> = ["--X", "X++", "X++"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let res = 1;
        assert_eq!(Solution::final_value_after_operations(operations), res);
    }

    #[test]
    fn test2() {
        let operations: Vec<_> = ["++X", "++X", "X++"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let res = 3;
        assert_eq!(Solution::final_value_after_operations(operations), res);
    }

    #[test]
    fn test3() {
        let operations: Vec<_> = ["X++", "++X", "--X", "X--"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let res = 0;
        assert_eq!(Solution::final_value_after_operations(operations), res);
    }
}
