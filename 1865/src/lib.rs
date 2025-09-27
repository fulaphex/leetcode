use std::collections;

struct FindSumPairs {
    arr1: Vec<i32>,
    arr2: Vec<i32>,
    freq2: collections::HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut freq = collections::HashMap::new();
        for n in nums2.iter() {
            freq.insert(*n, freq.get(&n).unwrap_or(&0) + 1);
        }
        println!("freq2: {:?}", freq);
        return Self {
            arr1: nums1,
            arr2: nums2,
            freq2: freq,
        };
    }

    fn add(&mut self, index: i32, val: i32) {
        let curr_val = self.arr2[index as usize];
        *self.freq2.get_mut(&curr_val).unwrap() -= 1;
        let new_val = curr_val + val;
        self.arr2[index as usize] = new_val;
        self.freq2
            .insert(new_val, self.freq2.get(&new_val).unwrap_or(&0) + 1);
        println!("freq: {:?}", self.freq2);
    }

    fn count(&self, tot: i32) -> i32 {
        let mut res = 0;
        for n in self.arr1.iter() {
            res += self.freq2.get(&(tot - n)).unwrap_or(&0);
        }
        return res as i32;
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (nums1, nums2) = (vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        // , [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]

        let mut obj = FindSumPairs::new(nums1, nums2);
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);

        // obj.add(index, val);
        // let ret_2: i32 = obj.count(tot);
    }
}
