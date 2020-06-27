use super::worker::Worker;
use std::collections::BinaryHeap;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

pub struct LoadBalancer {
    pub pool: BinaryHeap<Worker>, // resource pool binaryheap of workers
}

impl LoadBalancer {
    pub fn new(num: u32) -> Result<LoadBalancer, &'static str> {
        let mut pool = BinaryHeap::<Worker>::new();
        for i in 0..num {
            pool.push(Worker::new(i));
        }
        return Ok(LoadBalancer { pool: pool });
    }

    pub fn assign_task(&mut self, res: Sender<String>) {
        let mut worker = self.pool.pop().unwrap();
        worker.increase_workload();
        worker.do_work(res);
        self.pool.push(worker);
    }
}
