use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

struct Router {
    memory_limit: usize,
    offset_per_destination: HashMap<i32, usize>,
    // packet_per_destination_set: HashMap<i32, HashSet<(i32, i32)>>,
    packet_per_destination_list: HashMap<i32, Vec<(i32, i32)>>,
    packet_set: BTreeSet<(i32, i32, i32)>,
    fifo_que: VecDeque<(i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            memory_limit: memory_limit as usize,
            offset_per_destination: HashMap::new(),
            // packet_per_destination_set: HashMap::new(),
            packet_per_destination_list: HashMap::new(),
            packet_set: BTreeSet::new(),
            fifo_que: VecDeque::new(),
        }
    }
    fn _remove_packet(&mut self, source: i32, destination: i32, timestamp: i32) {
        // remove an existing packet from packet_set and packet_per_destination_set
        // leaving in fifo - this will be handled in forward_packet
        println!("removing packet {:?}", (source, destination, timestamp));
        self.packet_set.remove(&(source, destination, timestamp));
        // self.packet_per_destination_set
        //     .entry(destination)
        //     .or_default()
        //     .remove(&(source, timestamp));
        *self.offset_per_destination.entry(destination).or_insert(0) += 1;
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        println!(
            "adding packet {} -> {}; ts: {}",
            source, destination, timestamp
        );
        if self.packet_set.contains(&(source, destination, timestamp)) {
            return false;
        }
        if self.packet_set.len() == self.memory_limit {
            println!(
                "removing packet due to memory limit {}; current count: {}",
                self.memory_limit,
                self.packet_set.len()
            );
            self.forward_packet();
        }

        self.fifo_que.push_back((source, destination, timestamp));
        self.packet_set.insert((source, destination, timestamp));
        self.packet_per_destination_list
            .entry(destination)
            .or_default()
            .push((source, timestamp));
        // self.packet_per_destination_set
        //     .entry(destination)
        //     .or_insert(HashSet::new())
        //     .insert((source, timestamp));

        println!("packet set: {:?}", self.packet_set);
        // println!(
        //     "packet per destination set: {:?}",
        //     self.packet_per_destination_set
        // );
        println!("fifo que: {:?}", self.fifo_que);
        println!();

        return true;
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        // check if there's any packet in fifo_que that exists in packet_set
        while !self.fifo_que.is_empty() {
            let first_packet = self.fifo_que.pop_front().unwrap();
            if self.packet_set.contains(&first_packet) {
                self._remove_packet(first_packet.0, first_packet.1, first_packet.2);
                return vec![first_packet.0, first_packet.1, first_packet.2];
            }
        }
        return vec![];
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        // let destination_packets = &self.packet_per_destination_list.get(&destination).unwrap()[1..];
        if !self.packet_per_destination_list.contains_key(&destination) {
            return 0;
        }
        let destination_packets = &self.packet_per_destination_list.get(&destination).unwrap()
            [*self.offset_per_destination.get(&destination).unwrap_or(&0)..];
        println!("destination packets: {:?}", destination_packets);
        let s = [
            (0, 0),
            (2, 1),
            (4, 1),
            (5, 1),
            (3, 1),
            (1, 2),
            (2, 3),
            (4, 5),
            (5, 8),
            (3, 13),
            (1, 21),
            (2, 34),
            (4, 55),
        ];
        for j in 0..10 {
            let i = s.binary_search_by_key(&j, |&(a, b)| b);
            if let Err(x) = i {
                println!("{} {:?} {:?} {:?}", j, i, x, s[x]);
            } else if let Ok(x) = i {
                println!("{} {:?} {:?} {:?}", j, i, x, s[x]);
            } else {
                panic!();
            }
        }

        // let destination_packets = self.packet_per_destination_set.get(&destination).unwrap();
        let mut res = 0;
        for &(_source, timestamp) in destination_packets {
            if start_time <= timestamp && timestamp <= end_time {
                res += 1;
            }
        }
        return res;
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut router = Router::new(3);
        assert_eq!(router.add_packet(1, 4, 90), true);
        assert_eq!(router.add_packet(2, 5, 90), true);
        assert_eq!(router.add_packet(1, 4, 90), false);
        assert_eq!(router.add_packet(3, 5, 95), true);
        assert_eq!(router.add_packet(4, 5, 105), true);
        assert_eq!(router.forward_packet(), vec![2, 5, 90]);
        assert_eq!(router.add_packet(5, 2, 110), true);
        assert_eq!(router.get_count(5, 100, 110), 1);
    }
}
