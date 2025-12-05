use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_unstable_by_key(|ev| (ev[0], ev[1]));
        let (firstday, mut lastday) = (events[0][0], events[0][1]);
        for event in &events {
            lastday = lastday.max(event[1]);
        }

        let mut event_que = VecDeque::from(events);
        let mut end_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        let mut res = 0;

        for day in firstday..=lastday {
            // process events starting today and add ends to the heap
            while let Some(event) = event_que.front()
                && event[0] == day
            {
                let event = event_que.pop_front().unwrap();
                end_heap.push(Reverse(event[1]));
            }
            // remove ends before current day
            while let Some(Reverse(x)) = end_heap.peek()
                && *x < day
            {
                end_heap.pop();
            }
            // finish earliest ending event
            if !end_heap.is_empty() {
                end_heap.pop();
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
    fn test1() {
        let events = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 3;

        assert_eq!(Solution::max_events(events), res);
    }

    #[test]
    fn test2() {
        let events = [[1, 5], [1, 5], [1, 5], [2, 3], [2, 3]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 5;

        assert_eq!(Solution::max_events(events), res);
    }
}
