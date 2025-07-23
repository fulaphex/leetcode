struct Solution {}

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut res = 0;
        let rev = x < y;
        let (nx, ny) = (x.max(y), x.min(y));

        let (mut acount, mut bcount, mut a2count) = (0, 0, 0);
        for c in s.chars().chain([' ']) {
            let cc;
            if rev && (c == 'a' || c == 'b') {
                cc = if c == 'a' { 'b' } else { 'a' };
            } else {
                cc = c;
            }

            if cc == 'a' {
                if bcount == 0 {
                    acount += 1;
                } else {
                    a2count += 1;
                }
            } else if cc == 'b' {
                // c = 'b';
                if acount > 0 {
                    res += nx;
                    acount -= 1;
                } else if a2count > 0 {
                    res += nx;
                    a2count -= 1;
                } else {
                    // acount = 0; a2count = 0
                    bcount += 1;
                }
            } else {
                if bcount > 0 {
                    assert_eq!(acount, 0);
                }
                if acount > 0 {
                    assert_eq!(bcount, 0)
                }
                res += ny * bcount.min(a2count);
                (acount, bcount, a2count) = (0, 0, 0);
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("cdbcbbaaabab");
        let (x, y) = (4, 5);
        let res = 19;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }

    #[test]
    fn test2() {
        let s = String::from("aabbaaxybbaabb");
        let (x, y) = (5, 4);
        let res = 20;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }
}
