struct Solution {}

impl Solution {
    fn search<F>(start: i64, end: i64, func: F) -> i64
    where
        F: Fn(i64) -> bool,
    {
        if (end - start) == 1 {
            return end;
        }
        let mid = (start + end + 1) / 2;
        if !func(mid) {
            return Self::search(mid, end, func);
        } else {
            return Self::search(start, mid, func);
        }
    }

    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        if cars == 0 {
            return 0;
        }
        let check = |time: i64| {
            let car_counts = ranks
                .iter()
                .map(|&rank| ((time / rank as i64) as f64).sqrt() as i64)
                .sum::<i64>();
            return car_counts >= (cars as i64);
        };

        return Self::search(0, i64::MAX / 10, check);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ranks = vec![4, 2, 3, 1];
        let cars = 10;
        let res = 16;
        assert_eq!(Solution::repair_cars(ranks, cars), res);
    }

    #[test]
    fn test2() {
        let ranks = vec![5, 1, 8];
        let cars = 6;
        let res = 16;
        assert_eq!(Solution::repair_cars(ranks, cars), res);
    }
}
