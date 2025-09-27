use std::collections;

struct Solution {}

impl Solution {
    pub fn _inner2(
        strs: &[(i8, i8)],
        m: i8,
        n: i8,
        mem: &mut collections::HashMap<(usize, i8, i8), Option<i8>>,
    ) -> Option<i8> {
        if m.min(n) < 0 {
            return None;
        }
        if strs.len() == 0 {
            return Some(0);
        }
        let key = (strs.len(), m, n);
        if mem.contains_key(&key) {
            return *mem.get(&key).unwrap();
        }
        let (c0, c1) = strs[0];

        let mut res = Self::_inner2(&strs[1..], m, n, mem);
        let mut res1 = Self::_inner2(&strs[1..], m - c0, n - c1, mem);
        res1 = if res1.is_some() {
            Some(res1.unwrap() + 1)
        } else {
            res1
        };
        res = res.max(res1);
        mem.insert(key, res);
        return res;
    }
    #[inline]
    pub fn parse_str(s: &String) -> (i8, i8) {
        let (mut c0, mut c1) = (0, 0);
        for c in s.chars() {
            if c == '0' {
                c0 += 1;
            } else if c == '1' {
                c1 += 1;
            }
        }
        return (c0, c1);
    }
    pub fn find_max_form(mut strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut vals = vec![(0, 0); strs.len()];
        for (idx, s) in strs.iter().enumerate() {
            vals[idx] = Self::parse_str(s);
        }
        vals.sort();
        let mut mem = collections::HashMap::new();
        return Self::_inner2(&vals, m as i8, n as i8, &mut mem).unwrap_or(0) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs: Vec<String> = ["10", "0001", "111001", "1", "0"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let (m, n) = (5, 3);
        let res = 4;

        assert_eq!(Solution::find_max_form(strs, m, n), res);
    }

    #[test]
    fn test2() {
        let strs: Vec<String> = ["10", "1", "0"].iter().map(|&x| String::from(x)).collect();
        let (m, n) = (1, 1);
        let res = 2;

        assert_eq!(Solution::find_max_form(strs, m, n), res);
    }

    #[test]
    fn test3() {
        let strs: Vec<String> = [
            "11", "11", "0", "0", "10", "1", "1", "0", "11", "1", "0", "111", "11111000", "0",
            "11", "000", "1", "1", "0", "00", "1", "101", "001", "000", "0", "00", "0011", "0",
            "10000",
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect();
        let (m, n) = (90, 66);
        let res = 29;

        assert_eq!(Solution::find_max_form(strs, m, n), res);
    }
}
