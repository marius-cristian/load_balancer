pub mod client;
pub mod server;

use crate::client::send_work;
use crate::server::load_balancer;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let lb = load_balancer::LoadBalancer::new(5).unwrap();

    send_work(lb).await;
	// move occurs because `lb` has type `server::load_balancer::LoadBalancer`, which does not implement the `Copy` trait
    // send_work(lb);
}
