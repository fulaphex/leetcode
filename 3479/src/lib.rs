struct SegmentTree {
    arr: Vec<i32>,
    offset: usize,
}

impl SegmentTree {
    fn get_children(x: usize) -> (usize, usize) {
        return (2 * x + 1, 2 * x + 2);
    }
    pub fn from(v: &Vec<i32>) -> Self {
        let size = v.len();
        let log = (size as f32).log2();
        let offset = (1 << (log as i32 + 1)) - 1;
        let desired_size = (1 << (log as i32 + 2)) - 1;
        let mut arr = vec![-1; desired_size];
        for (idx, &val) in v.iter().enumerate() {
            arr[offset + idx] = val;
        }
        for i in (0..offset).rev() {
            let (left, right) = Self::get_children(i);
            arr[i] = arr[left].max(arr[right]);
        }
        return Self {
            arr: arr,
            offset: offset,
        };
    }
    pub fn remove(&mut self, idx: usize) {
        self.arr[idx + self.offset] = -1;
        let mut x = (self.offset + idx - 1) / 2;
        // updating the values up in the tree
        loop {
            let (left, right) = Self::get_children(x);
            self.arr[x] = self.arr[left].max(self.arr[right]);
            if x == 0 {
                break;
            }
            x = (x - 1) / 2;
        }
    }
    pub fn find(&self, val: i32) -> Option<usize> {
        if val > self.arr[0] {
            return None;
        }
        let mut idx = 0;
        while idx < self.offset {
            let (left, right) = Self::get_children(idx);
            if self.arr[left] >= val {
                idx = left;
            } else if self.arr[right] >= val {
                idx = right;
            } else {
                assert!(false);
            }
        }
        return Some(idx - self.offset);
    }
}

struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut tree = SegmentTree::from(&baskets);
        let mut res = fruits.len() as i32;
        for &fruit in fruits.iter() {
            let val = tree.find(fruit);
            if val.is_some() {
                tree.remove(val.unwrap());
                res -= 1;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        let res = 1;
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), res);
    }

    #[test]
    fn test2() {
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        let res = 0;
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), res);
    }
}
