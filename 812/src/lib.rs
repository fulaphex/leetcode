impl Solution {
    fn area_squared(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
        let a = (((p2[0] - p1[0]).pow(2) + (p2[1] - p1[1]).pow(2)) as f64).sqrt();
        let b = (((p3[0] - p2[0]).pow(2) + (p3[1] - p2[1]).pow(2)) as f64).sqrt();
        let c = (((p3[0] - p1[0]).pow(2) + (p3[1] - p1[1]).pow(2)) as f64).sqrt();
        let s = (a + b + c) / 2.;
        return s * (s - a) * (s - b) * (s - c);
    }
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut res: f64 = -1.;
        for (idx, p1) in points.iter().enumerate() {
            for (idx2, p2) in points[idx + 1..].iter().enumerate() {
                for p3 in &points[idx + 1..][idx2 + 1..] {
                    res = res.max(Self::area_squared(&p1, &p2, &p3));
                }
            }
        }
        return res.sqrt();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-5;

    #[test]
    fn test1() {
        let points = [[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2.;
        assert!((Solution::largest_triangle_area(points) - res).abs() <= EPS);
    }

    #[test]
    fn test2() {
        let points = [[1, 0], [0, 0], [0, 1]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 0.5;
        assert!((Solution::largest_triangle_area(points) - res).abs() <= EPS);
    }
}
