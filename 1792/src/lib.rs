use std::collections::BinaryHeap;

const EPSILON: f64 = 1. / 10_000.;
const MULT: i128 = 1_000_000_000_000_000_000;

// (p+1) / (t+1) - p/t
// (p+1)t / (t+1)t - p(t+1)/t(t+1)
// ((p+1)t - p(t+1)) / t(t+1)
// (pt+t - (pt+p)) / t(t+1)
// (t - p) / t(t+1)
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        let mut heap: BinaryHeap<(i128, i128, i128)> = BinaryHeap::new();

        fn get_ratio(pass: i128, total: i128) -> i128 {
            return MULT * (total - pass) / (total * (total + 1));
        }

        for class in classes {
            let (pass, total) = (class[0] as i128, class[1] as i128);
            heap.push((get_ratio(pass, total), pass, total));
        }

        while extra_students > 0 {
            let (_ratio, pass, total) = heap.pop().unwrap();
            heap.push((get_ratio(pass + 1, total + 1), pass + 1, total + 1));
            extra_students -= 1;
        }
        let sum_ratio: f64 = heap
            .iter()
            .map(|&(_ratio, pass, total)| (pass as f64 / total as f64))
            .sum::<f64>()
            .into();

        return sum_ratio / heap.len() as f64;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let classes = [[1, 2], [3, 5], [2, 2]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let extra_students = 2;
        let res = 0.78333;
        assert!((Solution::max_average_ratio(classes, extra_students) - res).abs() < EPSILON);
    }

    #[test]
    fn test2() {
        let classes = [[2, 4], [3, 9], [4, 5], [2, 10]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let extra_students = 4;
        let res = 0.53485;
        assert!((Solution::max_average_ratio(classes, extra_students) - res).abs() < EPSILON);
    }
}
