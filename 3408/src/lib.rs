use std::collections::{BTreeSet, HashMap};

struct TaskManager {
    task_to_user: HashMap<i32, i32>,
    task_to_priority: HashMap<i32, i32>,
    priority_set: BTreeSet<(i32, i32)>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut manager = Self {
            task_to_user: HashMap::new(),
            task_to_priority: HashMap::new(),
            priority_set: BTreeSet::new(),
        };
        for task in tasks {
            let (user_id, task_id, priority) = (task[0], task[1], task[2]);
            manager.add(user_id, task_id, priority);
        }

        return manager;
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_to_priority.insert(task_id, priority);
        self.task_to_user.insert(task_id, user_id);
        self.priority_set.insert((priority, task_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let prio = *self.task_to_priority.get(&task_id).unwrap();
        self.priority_set.remove(&(prio, task_id));
        self.priority_set.insert((new_priority, task_id));
        *self.task_to_priority.get_mut(&task_id).unwrap() = new_priority;
    }

    fn rmv(&mut self, task_id: i32) {
        let prio = *self.task_to_priority.get(&task_id).unwrap();
        self.priority_set.remove(&(prio, task_id));
        self.task_to_priority.remove(&task_id);
        self.task_to_user.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        let last_item = self.priority_set.last();
        if last_item.is_none() {
            return -1;
        }
        let task_id = last_item.unwrap().1;
        let user = *self.task_to_user.get(&task_id).unwrap();
        self.rmv(task_id);
        return user;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tasks = [[1, 101, 10], [2, 102, 20], [3, 103, 15]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let mut manager = TaskManager::new(tasks);
        manager.add(4, 104, 5);
        manager.edit(102, 8);
        assert_eq!(manager.exec_top(), 3);
        manager.rmv(101);
        manager.add(5, 105, 15);
        assert_eq!(manager.exec_top(), 5);
    }
}
