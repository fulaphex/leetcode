impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let (mut res, mut empty, mut full) = (0, 0, num_bottles);
        loop {
            if full == 0 {
                return res;
            }
            res += full;
            (full, empty) = ((empty + full) / num_exchange, (empty + full) % num_exchange);
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num_bottles = 9;
        let num_exchange = 3;
        let res = 13;
        assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
    }

    #[test]
    fn test2() {
        let num_bottles = 15;
        let num_exchange = 4;
        let res = 19;
        assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
    }
}
