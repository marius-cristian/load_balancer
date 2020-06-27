use std::rc::Rc;
use std::thread;

pub mod client;
pub mod server;

use crate::client::send_work;
use crate::server::load_balancer;

fn main() {
    let lb = load_balancer::LoadBalancer::new(5).unwrap();

    send_work(lb);
    // send_work(*lb);
}
