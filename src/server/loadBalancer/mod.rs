use super::worker::Worker;
use std::collections::BinaryHeap;

pub struct LoadBalancer {
    pub pool: BinaryHeap<Worker>, // resource pool binaryheap of workers
}

impl LoadBalancer {
    pub fn new(num: u32) -> Result<LoadBalancer, &'static str> {
        let mut pool = BinaryHeap::<Worker>::new();
        for i in 0..num {
            pool.push(Worker::new(i));
        }
        return Err("not implemented");
    }
}
