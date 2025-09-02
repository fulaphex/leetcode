impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut points: Vec<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();
        points.sort_by_key(|&(x, y)| (x, -y));
        for (ia, a) in points.iter().enumerate() {
            for (ib, b) in points.iter().enumerate().skip(ia + 1) {
                let mut add = true;
                if !(a.0 <= b.0 && a.1 >= b.1) {
                    continue;
                }
                for c in points.iter().take(ib).skip(ia + 1) {
                    if (a.0 <= c.0) && (c.0 <= b.0) && (a.1 >= c.1) && (c.1 >= b.1) {
                        add = false;
                        break;
                    }
                }
                if add {
                    res += 1;
                }
            }
        }
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let points = [[1, 1], [2, 2], [3, 3]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 0;
        assert_eq!(Solution::number_of_pairs(points), res);
    }

    #[test]
    fn test2() {
        let points = [[6, 2], [4, 4], [2, 6]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::number_of_pairs(points), res);
    }

    #[test]
    fn test3() {
        let points = [[3, 1], [1, 3], [1, 1]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::number_of_pairs(points), res);
    }

    #[test]
    fn test4() {
        let points = [[0, 1], [1, 3], [6, 1]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::number_of_pairs(points), res);
    }
}
