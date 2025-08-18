struct Solution {}
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let (un, uk, umax_pts) = (n as usize, k as usize, max_pts as usize);
        let mut dp = vec![0.; (n + 1) as usize];
        dp[0] = 1.;
        let mut window_sum = dp[0];
        for idx in 1..=un {
            dp[idx] = window_sum / (max_pts as f64);
            if idx < uk {
                window_sum += dp[idx];
            }
            if idx >= umax_pts {
                window_sum -= dp[idx - umax_pts];
            }
        }
        return dp[uk..=un].iter().sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (n, k, max_pts) = (10, 1, 10);
        let res = 1.;
        println!("{} {}", Solution::new21_game(n, k, max_pts), res);
        assert!((Solution::new21_game(n, k, max_pts) - res).abs() < 0.00001);
    }

    #[test]
    fn test2() {
        let (n, k, max_pts) = (6, 1, 10);
        let res = 0.6;
        println!("{} {}", Solution::new21_game(n, k, max_pts), res);
        assert!((Solution::new21_game(n, k, max_pts) - res).abs() < 0.00001);
    }

    #[test]
    fn test3() {
        let (n, k, max_pts) = (21, 17, 10);
        let res = 0.73278;
        println!("{} {}", Solution::new21_game(n, k, max_pts), res);
        assert!((Solution::new21_game(n, k, max_pts) - res).abs() < 0.00001);
    }
}
