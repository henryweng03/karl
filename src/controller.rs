use std::io;
use std::collections::HashSet;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bincode;
use mdns;
use tokio;
use futures_util::{pin_mut, stream::StreamExt};
use tokio::runtime::Runtime;

use crate::*;

/// The hostname of the devices we are searching for.
const SERVICE_NAME: &'static str = "karl._tcp._tcp.local";

/// Controller for interacting with mDNS.
pub struct Controller {
    rt: Runtime,
    hosts: Arc<Mutex<HashSet<SocketAddr>>>,
}

/// Connection to a host.
pub struct HostConnection {
    host: SocketAddr,
    stream: TcpStream,
}

impl Controller {
    /// Create a controller that asynchronously queries for new hosts
    /// at the given interval.
    pub fn new(rt: Runtime, interval: Duration) -> Self {
        let c = Controller { rt, hosts: Arc::new(Mutex::new(HashSet::new())) };
        let hosts = c.hosts.clone();
        c.rt.spawn(async move {
            let stream = mdns::discover::all(SERVICE_NAME, interval)
                .expect("TODO")
                .listen();
            pin_mut!(stream);
            while let Some(Ok(response)) = stream.next().await {
                // Find the port
                let port = if let Some(port) = response.port() {
                    port
                } else {
                    continue;
                };
                // Find the IPv4 address
                let ip_addr = response
                    .records()
                    .filter_map(|record| match record.kind {
                        mdns::RecordKind::A(ip_addr) => Some(ip_addr),
                        _ => None,
                    })
                    .next();
                // Form the socket address
                if let Some(ip_addr) = ip_addr {
                    let socket_addr = SocketAddr::new(ip_addr.into(), port);
                    debug!("discovered host: {:?}", socket_addr);
                    hosts.lock().unwrap().insert(socket_addr);
                }
            }
        });
        c
    }

    /// Find all hosts that broadcast the service over mDNS.
    pub fn find_hosts(&mut self) -> Vec<SocketAddr> {
        let mut hosts = self.hosts
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        hosts.sort();
        hosts
    }
}

impl HostConnection {
    /// Connect to a host.
    pub fn connect(host: SocketAddr) -> io::Result<Self> {
        info!("trying to connect to {:?}", host);
        let stream = TcpStream::connect(&host)?;
        Ok(HostConnection { host, stream })
    }

    /// Returns the address of the connected host.
    pub fn host_addr(&self) -> &SocketAddr {
        &self.host
    }

    /// Send a request to the connected host.
    fn send(&mut self, req: KarlRequest) -> Result<Option<KarlResult>, Error> {
        let bytes = bincode::serialize(&req)
            .map_err(|e| Error::SerializationError(format!("{:?}", e)))?;
        info!("sending {:?}...", req);
        write_packet(&mut self.stream, &bytes)?;
        info!("success!");

        // Wait for the response.
        info!("waiting for response...");
        let bytes = read_packet(&mut self.stream, true)?;
        let res = bincode::deserialize(&bytes)
            .map_err(|e| Error::SerializationError(format!("{:?}", e)))?;
        info!("done!");
        Ok(Some(res))
    }

    /// Ping the host.
    pub fn ping(&mut self) -> Result<Option<PingResult>, Error> {
        let req = KarlRequest::Ping(PingRequest::new());
        match self.send(req)? {
            Some(KarlResult::Ping(res)) => Ok(Some(res)),
            _ => Ok(None),
        }
    }

    /// Execute a compute request.
    pub fn execute(&mut self, req: ComputeRequest) -> io::Result<Option<ComputeResult>> {
        unimplemented!();
    }
}
