impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let (mut minx, mut miny, mut maxx, mut maxy) = (
            vec![usize::MAX; n + 2],
            vec![usize::MAX; n + 2],
            vec![0; n + 2],
            vec![0; n + 2],
        );
        for building in &buildings {
            let (x, y) = (building[0] as usize, building[1] as usize);
            minx[y] = minx[y].min(x);
            maxx[y] = maxx[y].max(x);
            miny[x] = miny[x].min(y);
            maxy[x] = maxy[x].max(y);
        }
        let mut res = 0;
        for building in &buildings {
            let (x, y) = (building[0] as usize, building[1] as usize);
            if minx[y] < x && x < maxx[y] && miny[x] < y && y < maxy[x] {
                res += 1;
            }
        }
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let buildings = [[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 1;
        assert_eq!(Solution::count_covered_buildings(n, buildings), res);
    }
}
