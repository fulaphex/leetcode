use std::collections;

struct Solution {}

impl Solution {
    fn servings(a: i32, b: i32, cache: &mut collections::HashMap<(i32, i32), f64>) -> f64 {
        let key = (a, b);
        let val = cache.get(&key);
        if val.is_none() {
            if a <= 0 && b <= 0 {
                return 0.5;
            } else if a <= 0 {
                return 1.;
            } else if b <= 0 {
                return 0.;
            } else {
                let (r1, r2, r3, r4) = (
                    Self::servings(a - 100, b, cache),
                    Self::servings(a - 75, b - 25, cache),
                    Self::servings(a - 50, b - 50, cache),
                    Self::servings(a - 25, b - 75, cache),
                );
                let res = (r1 + r2 + r3 + r4) / 4.;
                cache.insert(key, res);
            }
        }
        return cache.get(&key).unwrap().clone();
    }

    pub fn soup_servings(n: i32) -> f64 {
        if n > 5_000 {
            return 1.;
        }
        return Self::servings(n, n, &mut collections::HashMap::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 50;
        let res = 0.625;
        assert_eq!(Solution::soup_servings(n), res);
    }

    #[test]
    fn test2() {
        let n = 100;
        let res = 0.71875;
        assert_eq!(Solution::soup_servings(n), res);
    }

    #[test]
    fn test3() {
        let n = 100_000;
        let res = 1.;
        assert_eq!(Solution::soup_servings(n), res);
    }
}
