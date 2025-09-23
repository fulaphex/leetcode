impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1_terms = version1.split(".");
        let mut v2_terms = version2.split(".");
        loop {
            let (x_opt, y_opt) = (v1_terms.next(), v2_terms.next());
            if x_opt.is_none() && y_opt.is_none() {
                break;
            }
            let (x, y) = (
                x_opt.unwrap_or("0").parse::<i32>().unwrap(),
                y_opt.unwrap_or("0").parse::<i32>().unwrap(),
            );
            if x < y {
                return -1;
            } else if x > y {
                return 1;
            }
        }
        return 0;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (version1, version2) = (String::from("1.2"), String::from("1.10"));
        let res = -1;
        assert_eq!(Solution::compare_version(version1, version2), res);
    }

    #[test]
    fn test2() {
        let (version1, version2) = (String::from("1.01"), String::from("1.001"));
        let res = 0;
        assert_eq!(Solution::compare_version(version1, version2), res);
    }

    #[test]
    fn test3() {
        let (version1, version2) = (String::from("1.0"), String::from("1.0.0.0"));
        let res = 0;
        assert_eq!(Solution::compare_version(version1, version2), res);
    }
}
