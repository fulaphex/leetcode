use std::collections::VecDeque;

struct Map {
    grid: Vec<Vec<i32>>,
    rows: usize,
    columns: usize,
}

impl Map {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const HEALTHY: i32 = 0;
    const INFECTED: i32 = 1;
    const WALLEDOFF: i32 = 2;

    fn new(is_infected: Vec<Vec<i32>>) -> Self {
        // three types of field
        // 0 - healthy
        // 1 - infected
        // 2 - walled off
        return Map {
            rows: is_infected.len(),
            columns: is_infected[0].len(),
            grid: is_infected,
        };
    }

    fn get_neighs(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighs = vec![];
        for (dx, dy) in Self::DIRECTIONS {
            let (inx, iny) = (x as i32 + dx, y as i32 + dy);
            if !self.check_coord(inx, iny) {
                // out of bounds
                continue;
            }
            neighs.push((inx as usize, iny as usize));
        }

        return neighs;
    }

    fn check_coord(&self, x: i32, y: i32) -> bool {
        return (x >= 0) && (x < self.rows as i32) && (y >= 0) && (y < self.columns as i32);
    }

    fn find_region(&mut self) -> (i32, (usize, usize)) {
        // find the largest region
        let mut visited = vec![vec![false; self.columns]; self.rows];
        let mut regions: Vec<((usize, usize), i32, i32)> = vec![];

        for x in 0..self.rows {
            for y in 0..self.columns {
                if self.grid[x][y] == Self::INFECTED && !visited[x][y] {
                    let (mut expand, mut walls) = (0, 0);
                    let mut would_infect = vec![vec![false; self.columns]; self.rows];

                    // bfs to discover everything
                    let mut que = VecDeque::from([(x, y)]);
                    visited[x][y] = true;
                    while !que.is_empty() {
                        let (currx, curry) = que.pop_front().unwrap();

                        for (nx, ny) in self.get_neighs(currx, curry) {
                            if self.grid[nx][ny] == Self::HEALTHY {
                                // add a wall if that field isn't infected
                                walls += 1;
                                if !would_infect[nx][ny] {
                                    // expand to that field only once
                                    expand += 1;
                                    would_infect[nx][ny] = true;
                                }
                            }
                            if self.grid[nx][ny] == Self::INFECTED && !visited[nx][ny] {
                                visited[nx][ny] = true;
                                que.push_back((nx, ny));
                            }
                        }
                    }
                    // finished bfs
                    regions.push(((x, y), expand, walls));
                }
            }
        }
        let max_region = regions
            .into_iter()
            .max_by_key(|&(_xy, expand, _walls)| expand)
            .unwrap();
        let (point, _, walls) = max_region;
        // return the number of walls + a point in that region
        return (walls, point);
    }

    fn quarantine_region(&mut self, (x, y): (usize, usize)) {
        // given a point - mark the whole region as walled off
        let mut que = VecDeque::from([(x, y)]);
        let mut visited = vec![vec![false; self.columns]; self.rows];
        visited[x][y] = true;

        while !que.is_empty() {
            let (currx, curry) = que.pop_front().unwrap();
            self.grid[currx][curry] = Self::WALLEDOFF;
            for (nx, ny) in self.get_neighs(currx, curry) {
                if !visited[nx][ny] && self.grid[nx][ny] == Self::INFECTED {
                    que.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }
        }
    }

    fn expand_virus(&mut self) -> (bool, bool) {
        // iterate all the fields and expand the virus
        let (mut all_healthy, mut all_virus) = (true, true);
        let mut to_grow = vec![];

        for x in 0..self.rows {
            for y in 0..self.columns {
                if self.grid[x][y] == Self::WALLEDOFF {
                    continue;
                }
                if self.grid[x][y] == Self::INFECTED {
                    all_healthy = false;
                    continue;
                }
                all_virus = false;
                for (nx, ny) in self.get_neighs(x, y) {
                    if self.grid[nx][ny] == 1 {
                        to_grow.push((x, y));
                        break;
                    }
                }
            }
        }
        for (x, y) in to_grow {
            self.grid[x][y] = Self::INFECTED;
        }

        // return whether there's any virus left
        return (all_healthy, all_virus);
    }

    fn print(&self) {
        for row in &self.grid {
            println!("{:?}", row);
        }
        println!();
    }
}

impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        if is_infected
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .sum::<i32>()
            == 0
        {
            return 0;
        }
        let mut map = Map::new(is_infected);
        let (mut all_virus, mut all_healthy) = (false, false);
        let mut total_walls = 0;
        while !all_virus && !all_healthy {
            let (walls, point) = map.find_region();
            total_walls += walls;
            map.quarantine_region(point);
            (all_virus, all_healthy) = map.expand_virus();
        }
        return total_walls;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let is_infected = [
            [0, 1, 0, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 10;
        assert_eq!(Solution::contain_virus(is_infected), res);
    }

    #[test]
    fn test2() {
        let is_infected = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 4;
        assert_eq!(Solution::contain_virus(is_infected), res);
    }

    #[test]
    fn test3() {
        let is_infected = [
            [1, 1, 1, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1],
            [1, 1, 1, 0, 0, 0, 0, 0, 0],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 13;
        assert_eq!(Solution::contain_virus(is_infected), res);
    }

    #[test]
    fn test4() {
        let is_infected = [[0]].into_iter().map(|x| Vec::from(x)).collect();
        let res = 0;
        assert_eq!(Solution::contain_virus(is_infected), res);
    }
}
