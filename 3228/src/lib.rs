impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let (mut prev, mut res, mut one_seq) = (b'0', 0, 0);

        for c in s.bytes() {
            if c == b'1' {
                one_seq += 1;
            } else if prev == b'1' {
                res += one_seq;
            }
            prev = c;
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
        let s = String::from("1001101");
        let res = 4;

        assert_eq!(Solution::max_operations(s), res);
    }
}
