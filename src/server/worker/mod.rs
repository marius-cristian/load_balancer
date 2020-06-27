use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot::{Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub struct Worker {
    pub workload: u32,
    pub name: String,
    pub snd: Sender<Sender<String>>, 
    pub server: JoinHandle<u32>,
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
        let (tx,mut rx) = oneshot::channel();
        Worker {
            workload: 0,
            name: format!("My name is: {}", name),
            snd: tx,
            server: tokio::spawn(async move{
                    loop {
                        match rx.try_recv(){
                            Ok(res) => {
                               println!("Task received! name: {:?}", name);
                               thread::sleep(Duration::from_millis(2000));
                               println!("Task done! {:?}", name);
                               res.send(format!("Worker {} finished.", name)).unwrap();
                           },
                            _ => { },
                        }
                    }
                }),
        }
    }

    pub fn do_work(&self, res: Sender<String>) {

        self.snd.send(res).unwrap();
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
