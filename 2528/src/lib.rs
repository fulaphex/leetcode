use std::collections::VecDeque;

impl Solution {
    fn check(station_counts: &Vec<i64>, mut to_build: i64, radius: usize, target: i64) -> bool {
        // check if all stations can have power above `target` if `to_build` extra stations are
        // built
        let mut built = VecDeque::new();
        let mut curr_extra = 0;
        for (idx, stations) in station_counts.iter().enumerate() {
            while !built.is_empty() {
                let (first_location, first_stations) = built.front().unwrap();
                if idx > radius + first_location {
                    curr_extra -= first_stations;
                    built.pop_front();
                } else {
                    break;
                }
            }
            if stations + curr_extra < target {
                let needed = target - (stations + curr_extra);
                if needed > to_build {
                    return false;
                }
                to_build -= needed;
                let new_location = idx + radius; // building the station as far right as possible
                built.push_back((new_location, needed));
                curr_extra += needed;
            }
        }
        true
    }
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let to_build = k as i64;
        let radius = r as usize;

        let mut curr_sum = stations[..radius]
            .iter()
            .fold(0_i64, |acc, &x| acc + x as i64);
        let mut station_count = vec![0; stations.len()];

        for (idx, _el) in stations.iter().enumerate() {
            if idx + radius < stations.len() {
                curr_sum += stations[idx + radius] as i64;
            }
            if idx > radius {
                curr_sum -= stations[idx - radius - 1] as i64;
            }
            station_count[idx] = curr_sum;
        }
        let min_station = *station_count.iter().min().unwrap();
        let max_station = *station_count.iter().max().unwrap();

        let (mut start, mut end) = (min_station, max_station + to_build + 1);

        while start + 1 < end {
            let mid = (start + end + 1) / 2;
            if Self::check(&station_count, to_build, radius, mid) {
                start = mid;
            } else {
                end = mid;
            }
        }
        start
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let stations = vec![1, 2, 4, 5, 0];
        let r = 1;
        let k = 2;
        let res = 5;

        assert_eq!(Solution::max_power(stations, r, k), res);
    }

    #[test]
    fn test2() {
        let stations = vec![13, 12, 8, 14, 7];
        let r = 2;
        let k = 23;
        let res = 52;

        assert_eq!(Solution::max_power(stations, r, k), res);
    }
}
