impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut res = num_bottles;
        while num_exchange <= num_bottles {
            (num_bottles, res, num_exchange) =
                (num_bottles + 1 - num_exchange, res + 1, num_exchange + 1);
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
        let (num_bottles, num_exchange) = (13, 6);
        let res = 15;
        assert_eq!(Solution::max_bottles_drunk(num_bottles, num_exchange), res);
    }

    #[test]
    fn test2() {
        let (num_bottles, num_exchange) = (10, 3);
        let res = 13;
        assert_eq!(Solution::max_bottles_drunk(num_bottles, num_exchange), res);
    }
}
