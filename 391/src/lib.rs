use std::collections;

struct Solution {}

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let (mut minx, mut miny, mut maxx, mut maxy) = (
            rectangles[0][0],
            rectangles[0][1],
            rectangles[0][2],
            rectangles[0][3],
        );
        let mut area_sum = 0;
        let mut count = collections::HashMap::<(i32, i32), i32>::new();
        for rect in rectangles {
            for key in [
                (rect[0], rect[1]),
                (rect[0], rect[3]),
                (rect[2], rect[1]),
                (rect[2], rect[3]),
            ] {
                let val = count.get(&key).unwrap_or(&0);
                count.insert(key, val + 1);
            }
            minx = minx.min(rect[0]);
            miny = miny.min(rect[1]);
            maxx = maxx.max(rect[2]);
            maxy = maxy.max(rect[3]);
            area_sum += (rect[2] - rect[0]) as i64 * (rect[3] - rect[1]) as i64;
        }
        let area = (maxx - minx) as i64 * (maxy - miny) as i64;
        if area != area_sum {
            return false;
        }
        for key in [(minx, miny), (minx, maxy), (maxx, miny), (maxx, maxy)] {
            let val = count.remove(&key);
            if val.unwrap_or(0) != 1 {
                // println!("corner count != 1; {:?}", val);
                return false;
            }
        }
        for val in count.values() {
            if *val != 2 && *val != 4 {
                // println!("another corner count is odd: {:?}", val);
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let rectangles: Vec<Vec<i32>> = [
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [3, 2, 4, 4],
            [1, 3, 2, 4],
            [2, 3, 3, 4],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        assert_eq!(Solution::is_rectangle_cover(rectangles), true);
    }

    #[test]
    fn test2() {
        let rectangles: Vec<Vec<i32>> = [[1, 1, 3, 3], [3, 1, 4, 2], [1, 3, 2, 4], [2, 2, 4, 4]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        assert_eq!(Solution::is_rectangle_cover(rectangles), false);
    }
}
