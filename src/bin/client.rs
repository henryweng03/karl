#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::time::Duration;
use std::thread;

use tokio::runtime::Runtime;
use karl::{controller::{Controller, HostConnection}, ComputeRequest};

fn find_hosts(c: &mut Controller) -> Vec<SocketAddr> {
    loop {
        let hosts = c.find_hosts();
        if hosts.is_empty() {
            debug!("No hosts found! Try again in 1 second...");
            thread::sleep(Duration::from_secs(1));
        } else {
            return hosts;
        }
    };
}

/// Pings the host. Returns whether it is a success.
fn ping(host: SocketAddr) -> bool {
    info!("connecting to {:?}", host);
    let mut conn = HostConnection::connect(host).unwrap();
    info!("pinging {:?}", conn.host_addr());
    match conn.ping() {
        Ok(Some(_)) => return true,
        Ok(None) => warn!("could not be reached! (ping)"),
        Err(e) => error!("error pinging host: {:?}", e),
    }
    false
}

/// Requests computation from the host.
fn compute(host: SocketAddr) {
    info!("connecting to {:?}", host);
    let mut conn = HostConnection::connect(host).unwrap();
    info!("reading package.zip");
    let mut f = File::open("package.zip").expect("failed to open package.zip");
    let mut buffer: Vec<u8> = Vec::new();
    f.read(&mut buffer).expect("failed to read package.zip");
    info!("sending compute request");
    match conn.execute(ComputeRequest::new(buffer)) {
        Ok(Some(res)) => info!("Result: {:?}", res),
        Ok(None) => warn!("could not be reached! (compute)"),
        Err(e) => error!("error contacting host: {:?}", e),
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();
    let rt = Runtime::new().unwrap();
    let mut c = Controller::new(rt, Duration::from_secs(10));

    let hosts = find_hosts(&mut c);
    let host = hosts[0];  // Take the first host.
    if ping(host) {
        info!("ping!");
        compute(host);
    }
}
