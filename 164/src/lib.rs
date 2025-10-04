impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let (&min, &max) = (nums.iter().min().unwrap(), nums.iter().max().unwrap());
        if min == max {
            return 0;
        }

        let bucket_count = nums.len() as i32;
        let bucket_size = ((max - min + bucket_count) / bucket_count).max(1);
        let (mut bucket_min, mut bucket_max) = (
            vec![i32::MAX; bucket_count as usize],
            vec![i32::MIN; bucket_count as usize],
        );

        for &x in &nums {
            let bucket = ((x - min) / bucket_size) as usize;
            bucket_min[bucket] = bucket_min[bucket].min(x);
            bucket_max[bucket] = bucket_max[bucket].max(x);
        }
        let find = |bucket_min: &[i32]| -> usize {
            for (idx, &x) in bucket_min.iter().enumerate() {
                if x != i32::MAX {
                    return idx;
                }
            }
            return usize::MAX;
        };

        let (mut min_slice, mut max_slice) = (bucket_min.as_slice(), bucket_max.as_slice());

        let mut idx = find(&min_slice);
        let mut prev_max = max_slice[idx];
        let mut res = 0;
        (min_slice, max_slice) = (&min_slice[idx + 1..], &max_slice[idx + 1..]);

        loop {
            idx = find(&min_slice);
            if idx == usize::MAX {
                break;
            }
            let (curr_min, curr_max) = (min_slice[idx], max_slice[idx]);

            res = res.max(curr_min - prev_max);

            (min_slice, max_slice) = (&min_slice[idx + 1..], &max_slice[idx + 1..]);
            prev_max = curr_max;
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
        let nums = vec![3, 6, 9, 1];
        let res = 3;
        assert_eq!(Solution::maximum_gap(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![10];
        let res = 0;
        assert_eq!(Solution::maximum_gap(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 2];
        let res = 1;
        assert_eq!(Solution::maximum_gap(nums), res);
    }
}
