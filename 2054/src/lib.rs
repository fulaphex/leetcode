use std::collections::BinaryHeap;
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut start_time_heap = BinaryHeap::from_iter(events.iter().map(|e| (e[0], e[1], e[2])));
        let mut end_time_heap = BinaryHeap::from_iter(events.iter().map(|e| (e[1], e[0], e[2])));
        let mut res = events.iter().map(|e| e[2]).max().unwrap();
        let starting_latest = start_time_heap.pop().unwrap();

        let mut curr_value = starting_latest.2;

        // remove meetings that end >= start of curr
        while let Some(&(end, start, value)) = end_time_heap.peek()
            && end >= starting_latest.0
        {
            end_time_heap.pop();
        }

        while let Some(&(end, start, value)) = end_time_heap.peek() {
            while let Some(&(start2, end2, value)) = start_time_heap.peek()
                && start2 > end
            {
                curr_value = curr_value.max(value);
                start_time_heap.pop();
            }
            res = res.max(value + curr_value);
            end_time_heap.pop();
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
        let events = [[1, 3, 2], [4, 5, 2], [2, 4, 3]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 4;

        assert_eq!(Solution::max_two_events(events), res);
    }
}
