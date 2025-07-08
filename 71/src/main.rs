struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut res_parts: Vec<&str> = Vec::new();
        assert!(path.starts_with("/"));
        let parts: Vec<&str> = path.split("/").collect();
        println!("{:#?}", &parts[1..]);
        for part in &parts[1..] {
            if *part == "." {
                // ignoring the single dots
                continue;
            } else if *part == ".." {
                // if there's something to be removed - remove it
                if res_parts.len() > 0 {
                    res_parts.pop();
                }
                // if res_parts is empty - ignore double dots
            } else if part.len() > 0 {
                res_parts.push(part);
            }
        }
        println!("{:#?}", res_parts);
        let res = String::from("/".to_owned() + &res_parts.join("/"));
        return res;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let raw_path = String::from("/home/");
        let result = String::from("/home");
        assert_eq!(Solution::simplify_path(raw_path), result);
    }

    #[test]
    fn test_2() {
        let raw_path = String::from("/../");
        let result = String::from("/");
        assert_eq!(Solution::simplify_path(raw_path), result);
    }

    #[test]
    fn test_3() {
        let raw_path = String::from("/..");
        let result = String::from("/");
        assert_eq!(Solution::simplify_path(raw_path), result);
    }

    #[test]
    fn test_4() {
        let raw_path = String::from("/.../a/../b/c/../d/./");
        let result = String::from("/.../b/d");
        assert_eq!(Solution::simplify_path(raw_path), result);
    }

    #[test]
    fn test_5() {
        let raw_path = String::from("/home//foo/");
        let result = String::from("/home/foo");
        assert_eq!(Solution::simplify_path(raw_path), result);
    }

    #[test]
    fn test_6() {
        let raw_path = String::from("/home/user/Documents/../Pictures");
        let result = String::from("/home/user/Pictures");
        assert_eq!(Solution::simplify_path(raw_path), result);
    }
}
