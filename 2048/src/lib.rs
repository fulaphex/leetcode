impl Solution {
    fn check(mut n: i32) -> bool {
        let mut counts = [0; 10];
        while n > 0 {
            counts[(n % 10) as usize] += 1;
            n /= 10;
        }

        for (idx, &c) in counts.iter().enumerate() {
            if !(idx == c || c == 0) {
                return false;
            }
        }
        true
    }
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut curr = n + 1;
        loop {
            if Self::check(curr) {
                return curr;
            }
            curr += 1;
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 1;
        let res = 22;
        assert_eq!(Solution::next_beautiful_number(n), res);
    }
}
