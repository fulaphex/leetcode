impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let (mut x, mut res) = (1, 0);
        while x * x <= n {
            res += 1;
            x += 1;
        }
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let res = 1;
        assert_eq!(Solution::bulb_switch(n), res);
    }

    #[test]
    fn test2() {
        let n = 1;
        let res = 1;
        assert_eq!(Solution::bulb_switch(n), res);
    }

    #[test]
    fn test3() {
        let n = 0;
        let res = 0;
        assert_eq!(Solution::bulb_switch(n), res);
    }

    #[test]
    fn test4() {
        let n = 4;
        let res = 2;
        assert_eq!(Solution::bulb_switch(n), res);
    }

    #[test]
    fn test5() {
        let n = 200;
        let res =14;
        assert_eq!(Solution::bulb_switch(n), res);
    }
}
