use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot::Sender;

pub struct Worker {
    pub workload: u32,
    pub name: String,
    // the actual tokio thread
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.workload == other.workload
    }
}

impl Eq for Worker {}

// Flipped ordering in order to use a minheap in the loadbalancer
impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.workload == other.workload {
            return Ordering::Equal;
        } else if self.workload < other.workload {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.workload == other.workload {
            return Some(Ordering::Equal);
        } else if self.workload < other.workload {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

impl Worker {
    pub fn new(name: u32) -> Worker {
        Worker {
            workload: 0,
            name: format!("My name is: {}", name),
        }
    }

    pub fn do_work(&self, res: Sender<String>) {
        println!("Task received! name: {:?}", self.name);
        thread::sleep(Duration::from_millis(2000));
        println!("Task done! {:?}", self.name);
        res.send(format!("Worker {} finished.", self.name)).unwrap();
    }

    pub fn get_workload(&self) -> u32 {
        self.workload
    }
    pub fn increase_workload<'a>(&'a mut self) -> &'a u32 {
        self.workload += 1;
        &self.workload
    }

    pub fn decrease_workload<'a>(&'a mut self) -> &'a u32 {
        self.workload -= 1;
        &self.workload
    }
}
