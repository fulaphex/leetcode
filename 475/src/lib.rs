struct Solution {}

impl Solution {
    fn check(houses: &Vec<i32>, heaters: &Vec<i32>, n: i32) -> bool {
        let mut heater_iter = heaters.iter();
        let mut heater = *heater_iter.next().unwrap();
        for &house in houses {
            loop {
                // println!("{} {}", house, heater);
                let dist = (house - heater).abs();
                if dist <= n {
                    break;
                } else {
                    if heater > house {
                        return false;
                    }
                    let next_heater = heater_iter.next();
                    if next_heater.is_none() {
                        return false;
                    }
                    heater = *next_heater.unwrap();
                }
            }
        }
        return true;
    }
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort();
        heaters.sort();
        let minval = *houses.iter().min().min(heaters.iter().min()).unwrap();
        let maxval = *houses.iter().max().max(heaters.iter().max()).unwrap();
        let res = maxval - minval;
        let (mut left, mut right) = (-1, maxval - minval);
        let mut mid;
        loop {
            if left + 1 == right {
                return right;
            }
            mid = (left + right + 1) / 2;
            if Self::check(&houses, &heaters, mid) {
                (left, right) = (left, mid);
            } else {
                (left, right) = (mid, right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let houses = vec![1, 2, 3];
        let heaters = vec![2];
        let res = 1;
        assert_eq!(Solution::find_radius(houses, heaters), res);
    }

    #[test]
    fn test2() {
        let houses = vec![
            282475249, 622650073, 984943658, 144108930, 470211272, 101027544, 457850878, 458777923,
        ];
        let heaters = vec![
            823564440, 115438165, 784484492, 74243042, 114807987, 137522503, 441282327, 16531729,
            823378840, 143542612,
        ];
        let res = 161834419;
        assert_eq!(Solution::find_radius(houses, heaters), res);
    }
}
