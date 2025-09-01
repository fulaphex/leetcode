impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        // n = 1 -> 1.
        // n = 2 ->
        //   - 1 picks 1, 2 picks 2
        //   - 1 picks 2, 2 picks 1
        //
        // n ->
        //   - 1 picks 1 -> everyone picks their own
        //   - 1 picks n -> n cannot pick their
        //   - 1 picks k, 2..k-1 pick their own, k..n pick from (1, k+1..n)
        //                equivalent to f(n-k)
        // f(3) = 1/3 + 1/3*0 + 1/3 * f(2)
        // f(4) = 1/4 + 1/4*0 + 1/4 * f(3) + 1/4 * f(2)
        // f(n) = 1/n + 1/n*0 + 1/n * (f(n-1)..f(2))
        // f(n) = 1/n + 1/n * (n-2)/2 = 2/2n + (n-2)/2n = n/2n = 1/2
        return if n == 1 { 1. } else { 0.5 };
        fn f(n: i32) -> f64 {
            if n == 1 {
                return 1.;
            } else {
                return (1..=n - 1).map(|x| f(x)).sum::<f64>() / (n as f64);
            }
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 1;
        let res = 1.;
        assert_eq!(Solution::nth_person_gets_nth_seat(n), res);
    }

    #[test]
    fn test2() {
        let n = 2;
        let res = 0.5;
        assert_eq!(Solution::nth_person_gets_nth_seat(n), res);
    }
}
