use tokio::sync::mpsc;

use crate::server::load_balancer::LoadBalancer;

pub fn send_work(mut lb: LoadBalancer) {
    let (tx, mut rx) = mpsc::channel(100);
    lb.assign_task(tx);
    let received = rx.try_recv().unwrap();
    println!("Got: {}", received);
}
