use std::cmp;
use std::collections;

struct Solution {}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        return Self::most_booked_fast(n, meetings);
        // return Self::most_booked_slow(n, meetings);
    }

    pub fn most_booked_fast(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let un = n.try_into().unwrap();
        meetings.sort();
        println!("sorted meetings: {:?}", meetings);
        let mut meeting_count: Vec<i32> = vec![0; un];
        let mut free_rooms_heap: collections::BinaryHeap<cmp::Reverse<i32>> =
            collections::BinaryHeap::new();

        // values in the heap are (meeting_end_time, room_number)
        let mut meeting_end_times: collections::BinaryHeap<(cmp::Reverse<i64>, cmp::Reverse<i32>)> =
            collections::BinaryHeap::new();

        for x in 0..n {
            free_rooms_heap.push(cmp::Reverse(x));
        }
        println!("{:?}", free_rooms_heap);
        for meeting in meetings {
            let meeting_start: i64 = meeting[0].into();
            let meeting_end: i64 = meeting[1].into();
            let meeting_duration = meeting_end - meeting_start;
            println!("processing meeting {:?}", meeting);
            println!("meeting end times: {:?}", meeting_end_times);
            println!("meeting counts: {:?}", meeting_count);

            // remove the meetings that end before the current meeting start
            loop {
                let first_meeting = meeting_end_times.peek();
                if first_meeting.is_none() {
                    break;
                }
                let &first_meeting_val = first_meeting.unwrap();
                if first_meeting_val.0.0 <= meeting_start {
                    println!(
                        "removing meeting {:?}, it finishes before the current meeting {:?}",
                        first_meeting_val, meeting
                    );
                    meeting_end_times.pop();
                    free_rooms_heap.push(first_meeting_val.1);
                } else {
                    break;
                }
            }

            if free_rooms_heap.len() > 0 {
                // assign to the first free room
                let free_room_idx = free_rooms_heap.pop().unwrap();
                println!(
                    "assigning meeting {:?} to the free room {:?}",
                    meeting, free_room_idx
                );

                meeting_end_times.push((cmp::Reverse(meeting_end), free_room_idx));
                let room_idx: usize = free_room_idx.0.try_into().unwrap();
                meeting_count[room_idx] += 1;
                println!("meeting end times: {:?}", meeting_end_times);
            } else {
                // figure out the first room that finishes a meeting a assign that one
                assert!(meeting_end_times.len() > 0);
                let first_ending_meeting = meeting_end_times.pop().unwrap();
                println!(
                    "assigning meeting {:?} to the free room {:?}",
                    meeting, first_ending_meeting
                );

                meeting_end_times.push((
                    cmp::Reverse(first_ending_meeting.0.0 + meeting_duration),
                    first_ending_meeting.1,
                ));
                let room_idx: usize = first_ending_meeting.1.0.try_into().unwrap();
                meeting_count[room_idx] += 1;
                println!("meeting end times: {:?}", meeting_end_times);
            }
            println!();
        }
        println!("meeting counts: {:?}", meeting_count);

        let mut res: (usize, i32) = (0, meeting_count[0]);
        for (room_idx, &count) in meeting_count.iter().enumerate() {
            if count > res.1 {
                res = (room_idx, count);
            }
        }
        return res.0.try_into().unwrap();
    }
    
    pub fn most_booked_slow(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let un = n.try_into().unwrap();
        meetings.sort();
        println!("{:?}", meetings);
        let mut meeting_count: Vec<i32> = vec![0; un];
        let mut meeting_end_times: Vec<i64> = vec![0; un];

        for meeting in meetings {
            let meeting_start: i64 = meeting[0].into();
            let meeting_end: i64 = meeting[1].into();
            let meeting_duration = meeting_end - meeting_start;
            println!("{:?}", meeting_end_times);

            let mut free_room: Option<(usize, i64)> = None;
            // room that is free the earliest
            let mut best_room: (usize, i64) = (0, meeting_end_times[0]);

            for (room_idx, &room_end) in meeting_end_times.iter().enumerate() {
                if room_end <= meeting_start {
                    free_room = Some((room_idx, room_end));
                    break;
                }
                if room_end < best_room.1 {
                    best_room = (room_idx, room_end);
                }
            }
            if free_room.is_some() {
                let free_room_val = free_room.unwrap();
                println!("assigning meeting {:?} to {:?}", meeting, free_room);
                // there is a free room at the start time of the meeting
                meeting_end_times[free_room_val.0] = meeting_end;
                meeting_count[free_room_val.0] += 1;
            } else {
                // there is no free room at the start time of the meeting
                println!("assigning meeting {:?} to {:?}", meeting, best_room);
                // there is a free room at the start time of the meeting
                meeting_end_times[best_room.0] = best_room.1 + meeting_duration;
                meeting_count[best_room.0] += 1;
            }
        }
        println!("{:#?}", meeting_count);
        let mut res: (usize, i32) = (0, meeting_count[0]);
        for (room_idx, &count) in meeting_count.iter().enumerate() {
            if count > res.1 {
                res = (room_idx, count);
            }
        }
        return res.0.try_into().unwrap();
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let meetings = vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]];
        let res = 0;
        assert_eq!(Solution::most_booked(n, meetings), res);
    }

    #[test]
    fn test_2() {
        let n = 3;
        let meetings = vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]];
        let res = 1;
        assert_eq!(Solution::most_booked(n, meetings), res);
    }

    // #[test]
    // fn test_1() {
    //     let n = 2;
    //     let meetings = vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]];
    //     let res= 0 ;
    //     assert_eq!(Solution::most_booked(n, meetings), res);
    // }
}
