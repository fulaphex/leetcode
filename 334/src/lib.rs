impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first_smallest, mut second_smallest) = (i32::MAX, i32::MAX);
        for el in nums {
            if el <= first_smallest {
                first_smallest = el;
            } else if el <= second_smallest {
                second_smallest = el;
            } else if el > second_smallest {
                return true;
            }
        }

        false
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 5];
        let res = true;
        assert_eq!(Solution::increasing_triplet(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![5, 4, 3, 2, 1];
        let res = false;
        assert_eq!(Solution::increasing_triplet(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let res = true;
        assert_eq!(Solution::increasing_triplet(nums), res);
    }
}
