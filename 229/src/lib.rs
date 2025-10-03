impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut elements = vec![0; 2];
        let mut counts = vec![0; 2];
        for &n in &nums {
            let mut match_idx = None;
            let mut empty_idx = None;
            for (idx, (&el, &count)) in std::iter::zip(&elements, &counts).enumerate() {
                if n == el {
                    match_idx = Some(idx);
                }
                if count == 0 {
                    empty_idx = Some(idx);
                }
            }
            if match_idx.is_some() {
                counts[match_idx.unwrap()] += 1;
            } else if empty_idx.is_some() {
                elements[empty_idx.unwrap()] = n;
                counts[empty_idx.unwrap()] += 1;
            } else {
                for c in counts.iter_mut() {
                    *c -= 1;
                }
            }
        }

        let mut counts = vec![0; 2];
        for &n in &nums {
            for (x, count) in std::iter::zip(elements.iter_mut(), counts.iter_mut()) {
                if n == *x {
                    *count += 1;
                    break;
                }
            }
        }

        return std::iter::zip(elements, counts)
            .filter(|&(_el, count)| count > nums.len() / 3)
            .map(|(el, _count)| el)
            .collect();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        let res = vec![3];
        assert_eq!(Solution::majority_element(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1];
        let res = vec![1];
        assert_eq!(Solution::majority_element(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2];
        let res = vec![1, 2];
        assert_eq!(Solution::majority_element(nums), res);
    }

    #[test]
    fn test4() {
        let nums = vec![2, 1, 1, 3, 1, 4, 5, 6];
        let res = vec![1];
        assert_eq!(Solution::majority_element(nums), res);
    }
}
