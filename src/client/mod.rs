use tokio::sync::oneshot;

use crate::server::load_balancer::LoadBalancer;

pub fn send_work(mut lb: LoadBalancer) {
    let (tx, mut rx) = oneshot::channel();
    lb.assign_task(tx);
    let received = rx.try_recv().unwrap();
    println!("Got: {}", received);
}
