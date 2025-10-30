struct Solve {
    target: Vec<i32>,
    mintree: Vec<(i32, usize)>,
    offset: usize,
}

impl Solve {
    fn new(target: Vec<i32>) -> Self {
        let offset = 2_usize.pow((target.len() - 1).ilog2() + 1);
        let mut mintree = vec![(i32::MAX, usize::MAX); 2 * offset + 2];

        for (idx, &el) in target.iter().enumerate() {
            mintree[offset + idx] = (el, idx);
        }

        for idx in (1..offset).rev() {
            mintree[idx] = mintree[2 * idx + 1].min(mintree[2 * idx]);
        }

        Self {
            target,
            mintree,
            offset,
        }
    }

    fn interval_min(&self, mut start: usize, mut end: usize) -> (i32, usize) {
        let mut res = (i32::MAX, usize::MAX);
        start += self.offset;
        end += self.offset;

        while start < end {
            res = res.min(self.mintree[start]);
            if start % 2 == 1 {
                start += 1;
            }
            res = res.min(self.mintree[end]);
            if end % 2 == 0 {
                end -= 1;
            }
            start /= 2;
            end /= 2;
        }

        if start == end {
            res = res.min(self.mintree[start]);
        }
        res
    }
    fn solve_recursive(&self, start_idx: usize, end_idx: usize, height: i32) -> i32 {
        let (min_val, min_idx) = self.interval_min(start_idx, end_idx);
        let mut res = min_val - height;

        if min_idx > start_idx {
            res += self.solve_recursive(start_idx, min_idx - 1, min_val);
        }
        if min_idx < end_idx {
            res += self.solve_recursive(min_idx + 1, end_idx, min_val);
        }

        res
    }

    fn solve(&self) -> i32 {
        self.solve_recursive(0, self.target.len() - 1, 0)
    }
}

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        (1..target.len()).fold(target[0], |acc, idx| {
            acc + (target[idx] - target[idx - 1]).max(0)
        })
        // Solve::new(target).solve()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let target = vec![1, 2, 3, 2, 1];
        let res = 3;
        assert_eq!(Solution::min_number_operations(target), res);
    }

    #[test]
    fn test2() {
        let target = vec![1, 2, 3, 2];
        let res = 3;
        assert_eq!(Solution::min_number_operations(target), res);
    }
}
