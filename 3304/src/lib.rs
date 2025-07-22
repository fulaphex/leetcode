struct Solution {}

impl Solution {
    pub fn kth_character(mut k: i32) -> char {
        let ku = (k - 1) as u32;
        let res = ku.count_ones() % 26;
        return (('a' as u8) + (res as u8)) as char;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let k = 5;
        let res = 'b';
        assert_eq!(Solution::kth_character(k), res);
    }

    #[test]
    fn test2() {
        let k = 10;
        let res = 'c';
        assert_eq!(Solution::kth_character(k), res);
    }
}
