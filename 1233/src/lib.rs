struct Solution {}
impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        folder.sort();
        for f in folder {
            if res.len() == 0 {
                res.push(f);
            } else {
                let last_el = &res[res.len() - 1];
                if f.len() <= last_el.len() {
                    res.push(f);
                } else {
                    if *last_el == f[..last_el.len()] && f[last_el.len()..last_el.len() + 1] == *"/"
                    {
                        continue;
                    } else {
                        res.push(f);
                    }
                }
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
        let folder = vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let res = vec!["/a", "/c/d", "/c/f"];
        assert_eq!(Solution::remove_subfolders(folder), res);
    }

    #[test]
    fn test2() {
        let folder = vec!["/a", "/a/b/c", "/a/b/d"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let res = vec!["/a"];
        assert_eq!(Solution::remove_subfolders(folder), res);
    }

    #[test]
    fn test3() {
        let folder = vec!["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let res = vec!["/a/b/c", "/a/b/ca", "/a/b/d"];
        assert_eq!(Solution::remove_subfolders(folder), res);
    }
}
