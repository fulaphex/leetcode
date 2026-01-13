impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let squares = squares
            .into_iter()
            .map(|sq| (sq[1] as f64, sq[2] as f64))
            .collect::<Vec<_>>();

        let (mut low, mut hi) = (f64::MAX, f64::MIN);
        for &(y, l) in &squares {
            low = low.min(y);
            hi = hi.max(y + l);
        }

        while hi - low > 1e-5 {
            let mid = (hi + low) / 2.;
            let (mut below, mut above) = (0., 0.);
            for &(y, l) in &squares {
                let up = (y + l).min(mid);
                let bottom = (y).max(mid);
                below += (up - y).max(0.) * l;
                above += (y + l - bottom).max(0.) * l;
            }
            if above > below {
                low = mid;
            } else {
                hi = mid;
            }
        }

        low
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    const DELTA: f64 = 1e-5;

    #[test]
    fn test() {
        let squares = [[0, 0, 1], [2, 2, 1]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 1.00000;

        assert!((Solution::separate_squares(squares) - res).abs() < DELTA);
    }
}
