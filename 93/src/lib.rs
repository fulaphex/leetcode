impl Solution {
    fn solve(chars: &[u8], acc: &mut Vec<i32>, res: &mut Vec<String>) {
        if acc.len() == 4 {
            if chars.is_empty() {
                let x = acc.iter().map(|&x| x.to_string()).collect::<Vec<_>>();
                res.push(x.join("."));
            }
            return;
        }
        for part_len in 1..=(3.min(chars.len())) {
            if part_len > 1 && chars[0] == b'0' {
                continue;
            }
            let mut part = 0;
            for &x in &chars[..part_len] {
                part = 10 * part + (x - b'0') as i32;
            }
            if part >= 256 {
                continue;
            }
            acc.push(part);
            Self::solve(&chars[part_len..], acc, res);
            acc.pop();
        }
    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = vec![];
        let chars: Vec<_> = s.bytes().collect();
        Self::solve(&chars, &mut vec![], &mut res);
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("25525511135");
        let res = ["255.255.11.135", "255.255.111.35"];
        assert_eq!(Solution::restore_ip_addresses(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("0000");
        let res = ["0.0.0.0"];
        assert_eq!(Solution::restore_ip_addresses(s), res);
    }

    #[test]
    fn test3() {
        let s = String::from("101023");
        let res = [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ];
        assert_eq!(Solution::restore_ip_addresses(s), res);
    }
}
