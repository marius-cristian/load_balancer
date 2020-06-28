pub mod client;
pub mod server;
use tokio::sync::mpsc;

use crate::client::send_work;
use crate::server::load_balancer;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let mut lb = load_balancer::LoadBalancer::new(5).unwrap();
    
     // Simulated clients, as couldnt make the client work
    let (tx, mut rx) = mpsc::channel(100);
    let (tx1, mut rx1) = mpsc::channel(100);
    let (tx2, mut rx2) = mpsc::channel(100);
    let (tx3, mut rx3) = mpsc::channel(100);
    let (tx4, mut rx4) = mpsc::channel(100);
    let (tx5, mut rx5) = mpsc::channel(100);

    lb.assign_task(tx).await;
    lb.assign_task(tx1).await;
    lb.assign_task(tx2).await;
    lb.assign_task(tx3).await;
    lb.assign_task(tx4).await;
    lb.assign_task(tx5).await;


    let received = rx.recv().await.unwrap();
    let received1 = rx1.recv().await.unwrap();
    let received2 = rx2.recv().await.unwrap();
    let received3 = rx3.recv().await.unwrap();
    let received4 = rx4.recv().await.unwrap();
    let received5 = rx5.recv().await.unwrap();


    println!("Got: {}", received);
    println!("Got: {}", received1);
    println!("Got: {}", received2);
    println!("Got: {}", received3);
    println!("Got: {}", received4);
    println!("Got: {}", received5);


    // Alternative using the client send work; but there is an error
    // send_work(lb);
	// move occurs because `lb` has type `server::load_balancer::LoadBalancer`, which does not implement the `Copy` trait
    // send_work(lb);
    // send_work(lb);
    // send_work(lb);
}
