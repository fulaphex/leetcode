impl Solution {
    fn search(arr: &[i32], x: i32) -> usize {
        // return the number of elements <= x
        if arr.len() == 0 {
            return 0;
        }
        if arr.len() == 1 {
            return if arr[0] <= x { 1 } else { 0 };
        }
        let mid = (arr.len() + 1) / 2;
        if arr[mid] <= x {
            return mid + Self::search(&arr[mid..], x);
        } else {
            return Self::search(&arr[..mid], x);
        }
    }

    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;

        for idx1 in 0..nums.len() {
            for idx2 in idx1 + 1..nums.len() {
                let (a, b) = (nums[idx1], nums[idx2]);
                // println!("idx12: {:?}", (idx1, idx2));
                let limit = a + b - 1;
                let rest = &nums[idx2 + 1..];
                let inc = Self::search(rest, limit);
                // println!("a: {}, b: {}; rest: {:?}", a, b, rest);
                // println!("inc: {}", inc);
                res += inc as i32;
            }
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
        let nums = vec![2, 2, 3, 4];
        let res = 3;
        assert_eq!(Solution::triangle_number(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 2, 3, 4];
        let res = 4;
        assert_eq!(Solution::triangle_number(nums), res);
    }
}
