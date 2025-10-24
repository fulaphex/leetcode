impl Solution {
    fn invert_mod(a: usize, modulus: usize) -> usize {
        assert!(a != 0);
        std::iter::repeat_n(a, modulus - 2).product()
    }

    fn n_choice_k(n: i32, k: i32, modulus: i32) -> i32 {
        let k = if k > n / 2 { n - k } else { k };
        let (n, k, modulus) = (n as u128, k as u128, modulus as u128);
        let num = (n - k + 1..=n).product::<u128>();
        let denom = (1..=k).product::<u128>();
        ((num / denom) % modulus) as i32
    }

    fn pascal_row(n: usize, modulus: usize) -> Vec<usize> {
        let mut fact = vec![(0, 1); n + 1];
        for i in 1..=n {
            let (prev_div, prev_rem) = fact[i - 1];
            let mut div = 0;
            let mut x = i;
            while x % modulus == 0 {
                div += 1;
                x /= modulus;
            }
            fact[i] = (prev_div + div, (prev_rem * x) % modulus);
        }

        let mut res = vec![0; n + 1];
        let num = fact[n];

        for i in 0..=n.div_ceil(2) {
            let denom = {
                let (x, y) = (fact[i], fact[n - i]);
                (x.0 + y.0, (x.1 * y.1) % modulus)
            };
            assert!(num.0 >= denom.0);

            let divided = (
                num.0 - denom.0,
                (num.1 * Self::invert_mod(denom.1, modulus)) % modulus,
            );
            res[i] = if divided.0 > 0 { 0 } else { divided.1 };
            res[n - i] = res[i];
        }

        res
    }

    pub fn has_same_digits(s: String) -> bool {
        let digits: Vec<_> = s.as_bytes().iter().map(|&x| (x - b'0') as usize).collect();
        let primes = [2, 5];

        for modulus in primes {
            let (mut first, mut second) = (0, 0);
            let pascal = Self::pascal_row(digits.len() - 2, modulus);

            for (idx, &mult) in pascal.iter().enumerate() {
                first += digits[idx] * mult;
                first %= modulus;
                second += digits[idx + 1] * mult;
                second %= modulus;
            }

            if first != second {
                return false;
            }
        }
        true
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("3902");
        let res = true;
        assert_eq!(Solution::has_same_digits(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("34789");
        let res = false;
        assert_eq!(Solution::has_same_digits(s), res);
    }
}
