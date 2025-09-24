impl Solution {
    pub fn fraction_to_decimal(mut numerator: i32, mut denominator: i32) -> String {
        if numerator == 0 {
            return String::from("0");
        }
        let (mut numerator, mut denominator) = (numerator as i64, denominator as i64);
        let mut res = String::new();

        // figuring whether the result is negative
        if denominator < 0 {
            numerator *= -1;
            denominator *= -1;
        }
        if numerator < 0 {
            res += &String::from("-");
            numerator *= -1;
        }

        // computing integer
        assert!(numerator > 0);
        assert!(denominator > 0);

        let integer = numerator / denominator;
        res += &integer.to_string();
        if numerator % denominator == 0 {
            return res;
        }
        res += ".";
        loop {
            if numerator % 2 == 0 && denominator % 2 == 0 {
                numerator /= 2;
                denominator /= 2;
            } else if numerator % 5 == 0 && denominator % 5 == 0 {
                numerator /= 5;
                denominator /= 5;
            } else {
                break;
            }
        }

        // computing frac
        let (mut div2, mut den2, mut div5, mut den5) = (0, denominator, 0, denominator);
        while den2 % 2 == 0 {
            div2 += 1;
            den2 /= 2;
        }
        while den5 % 5 == 0 {
            div5 += 1;
            den5 /= 5;
        }
        let frac_len = div2.max(div5);

        let mut frac = String::new();
        numerator %= denominator;
        for _ in 0..frac_len {
            frac += &(10 * numerator / denominator).to_string();
            numerator = (10 * numerator) % denominator;
        }
        res += &frac;

        // computing repeating
        if numerator == 0 {
            return res;
        } else {
            let mut num = 10 * numerator;
            let mut repeating_str = String::new();
            while (num - numerator) % denominator != 0 {
                repeating_str += &(num / denominator).to_string();
                num = 10 * (num % denominator);
            }
            repeating_str += &(num / denominator).to_string();

            return res + "(" + &repeating_str + ")";
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (numerator, denominator) = (1, 2);
        let res = String::from("0.5");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }

    #[test]
    fn test2() {
        let (numerator, denominator) = (2, 1);
        let res = String::from("2");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }

    #[test]
    fn test3() {
        let (numerator, denominator) = (4, 333);
        let res = String::from("0.(012)");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }

    #[test]
    fn test4() {
        let (numerator, denominator) = (1, 12);
        let res = String::from("0.08(3)");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }

    #[test]
    fn test5() {
        let (numerator, denominator) = (22, 7);
        let res = String::from("3.(142857)");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }

    #[test]
    fn test6() {
        let (numerator, denominator) = (50, -8);
        let res = String::from("-6.25");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }

    #[test]
    fn test7() {
        let (numerator, denominator) = (-1, -2147483648);
        let res = String::from("0.0000000004656612873077392578125");
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    }
}
