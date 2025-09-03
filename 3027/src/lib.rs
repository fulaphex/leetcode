impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut points: Vec<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();
        points.sort_by_key(|&(x, y)| (x, -y));

        #[inline]
        fn upleft((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> bool {
            // is a up-left of b
            return ay >= by;
        }

        for (ia, &a) in points.iter().enumerate() {
            let mut highest = i32::MIN;
            for (idx, w) in points.windows(2).skip(ia).enumerate() {
                let (c, b) = (w[0], w[1]);
                if highest == a.1 {
                    break;
                }

                if c.1 <= a.1 && idx != 0 {
                    highest = highest.max(c.1);
                }

                if !upleft(a, b) {
                    continue;
                }
                if highest < b.1 {
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
