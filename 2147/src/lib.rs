impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: usize = 1_000_000_007;
        let (mut seat_count, mut res, mut prev_seat) = (0, 1, 0);
        for (idx, c) in corridor.bytes().enumerate() {
            if c == b'S' {
                if seat_count % 2 == 0 && seat_count > 0 {
                    res = (res * (idx - prev_seat)) % MOD;
                }
                prev_seat = idx;
                seat_count += 1;
            }
        }

        if seat_count % 2 == 1 || seat_count == 0 {
            0
        } else {
            res as i32
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let corridor = String::from("SSPPSPS");
        let res = 3;
        assert_eq!(Solution::number_of_ways(corridor), res);
    }
}
