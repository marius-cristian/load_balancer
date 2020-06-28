use tokio::sync::mpsc;

use crate::server::load_balancer::LoadBalancer;

pub async fn send_work(mut lb: LoadBalancer) {
    let (tx, mut rx) = mpsc::channel(100);
    lb.assign_task(tx).await;
    let received = rx.recv().await.unwrap();
    println!("Got: {}", received);
}
