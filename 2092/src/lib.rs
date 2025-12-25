use std::collections::HashMap;
use std::collections::HashSet;

struct UnionFind {
    representative: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            representative: (0..n).collect(),
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.representative[n] == n {
            n
        } else {
            let r = self.find(self.representative[n]);
            self.representative[n] = r;
            r
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let (rx, ry) = (self.find(x), self.find(y));
        self.representative[rx] = ry;
    }
}

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let n = n as usize;
        let first_person = first_person as usize;

        let mut meetings_per_time = HashMap::new();

        for meeting in meetings {
            meetings_per_time
                .entry(meeting[2])
                .or_insert(vec![])
                .push((meeting[0] as usize, meeting[1] as usize));
        }

        let mut times = meetings_per_time.keys().collect::<Vec<_>>();
        times.sort_unstable();

        let mut uf = UnionFind::new(n);
        uf.union(0, first_person);

        for time in times {
            let time_meetings = meetings_per_time.get(time).unwrap();
            let mut people = HashSet::new();

            let know_repr = uf.find(0);

            for &(x, y) in time_meetings {
                let (rx, ry) = (uf.find(x), uf.find(y));
                if rx != ry {
                    uf.union(x, y);
                    people.insert(x);
                    people.insert(y);
                }
            }

            let know_repr = uf.find(0);

            for &x in &people {
                if uf.find(x) != know_repr {
                    uf.representative[x] = x;
                }
            }
        }

        let know_repr = uf.find(0);

        (0..n)
            .filter(|&x| uf.find(x) == know_repr)
            .map(|x| x as i32)
            .collect::<Vec<_>>()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 6;
        let meetings = [[1, 2, 5], [2, 3, 8], [1, 5, 10]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let first_person = 1;
        let res = vec![0, 1, 2, 3, 5];

        assert_eq!(Solution::find_all_people(n, meetings, first_person), res);
    }
}
