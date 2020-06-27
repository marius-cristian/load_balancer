use std::cmp::Ordering;

pub struct Worker {
    pub workload: u32,
    // the actual tokio worker
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
    pub fn new() -> Worker {
        Worker { workload: 0 }
    }

    pub fn do_work() {}

    pub fn get_workload(&self) -> u32 {
        self.workload
    }

    pub fn increase_workload(&self) -> u32 {
        self.workload += 1;
        self.workload
    }

    pub fn decrease_workload(&self) -> u32 {
        self.workload -= 1;
        self.workload
    }
}
