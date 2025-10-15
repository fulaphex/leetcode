impl Solution {
    // full tree-building simulation
    // fn is_valid(arr: &[&str]) -> (bool, usize) {
    //     let hash: &str = "#";
    //     if arr.is_empty() {
    //         return (false, 0);
    //     } else if arr[0] == hash {
    //         return (true, 1);
    //     } else {
    //         let (left_res, left_len) = Self::is_valid(&arr[1..]);
    //         if !left_res {
    //             return (false, 0);
    //         }
    //         let (right_res, right_len) = Self::is_valid(&arr[1 + left_len..]);
    //         return (right_res, 1 + left_len + right_len);
    //     }
    // }
    //
    // pub fn is_valid_serialization(preorder: String) -> bool {
    //     let arr: Vec<_> = preorder.split(",").collect();
    //     let res = Self::is_valid(arr.as_slice());
    //     res.0 && res.1 == arr.len()
    // }

    // simple counting
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut slots = 1;
        for x in preorder.split(",") {
            slots -= 1;
            if slots < 0 {
                return false;
            }
            if x != "#" {
                slots += 2;
            }
        }
        slots == 0
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let preorder = String::from("9,3,4,#,#,1,#,#,2,#,6,#,#");
        let res = true;
        assert_eq!(Solution::is_valid_serialization(preorder), res);
    }

    #[test]
    fn test2() {
        let preorder = String::from("9,#");
        let res = false;
        assert_eq!(Solution::is_valid_serialization(preorder), res);
    }

    #[test]
    fn test3() {
        let preorder = String::from("9,#,#,1");
        let res = false;
        assert_eq!(Solution::is_valid_serialization(preorder), res);
    }

    #[test]
    fn test4() {
        let preorder = String::from("9,#,#");
        let res = true;
        assert_eq!(Solution::is_valid_serialization(preorder), res);
    }
}
