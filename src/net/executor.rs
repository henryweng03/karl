//! The client for interacting with the Karl controller and services.
//!
//! Each language SDK must re-implement its own client, called an executor.
//! The interactions with other Karl entities are entirely through TCP.
//! The interface must follow the request and result interface defined in
//! `common.rs`.
//!
//! If the request to a host is unsuccessful, the client must query the
//! executor for a different host and try again on the client-side.
//! Addresses are passed in the form of `<IP>:<PORT>`.
use std::net::TcpStream;
use protobuf;
use protobuf::Message;
use crate::protos;
use crate::packet;
use crate::common::*;

/// Register an IoT client with the controller.
pub fn register_client(controller_addr: &str, id: &str) {
    let mut stream = TcpStream::connect(controller_addr).unwrap();
    let mut req = protos::RegisterRequest::default();
    req.set_id(id.to_string());
    let req_bytes = req
        .write_to_bytes()
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    packet::write(&mut stream, HT_REGISTER_REQUEST, &req_bytes).unwrap();
    let (header, _) = &packet::read(&mut stream, 1).unwrap()[0];
    assert_eq!(header.ty, HT_REGISTER_RESULT);
}

/// Returns a host address given by the controller.
pub fn get_host(controller_addr: &str) -> String {
    // TODO: handle network errors with connecting, writing, reading.
    // Deserialization may also fail due to the other side.
    let mut stream = TcpStream::connect(controller_addr).unwrap();
    let req = protos::HostRequest::default();
    let req_bytes = req
        .write_to_bytes()
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    packet::write(&mut stream, HT_HOST_REQUEST, &req_bytes).unwrap();
    let (header, res_bytes) = &packet::read(&mut stream, 1).unwrap()[0];
    assert_eq!(header.ty, HT_HOST_RESULT);
    let res = protobuf::parse_from_bytes::<protos::HostResult>(&res_bytes)
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    format!("{}:{}", res.get_ip(), res.get_port())
}

/// Pings the given host and returns the result.
pub fn send_ping(host: &str) -> protos::PingResult {
    let mut stream = TcpStream::connect(&host).unwrap();
    let req = protos::PingRequest::default();
    let req_bytes = req
        .write_to_bytes()
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    packet::write(&mut stream, HT_PING_REQUEST, &req_bytes).unwrap();
    let (header, res_bytes) = &packet::read(&mut stream, 1).unwrap()[0];
    assert_eq!(header.ty, HT_PING_RESULT);
    protobuf::parse_from_bytes::<protos::PingResult>(&res_bytes)
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap()
}

/// Sends a compute request to the given host and returns the result.
pub fn send_compute(
    host: &str,
    req: protos::ComputeRequest,
) -> protos::ComputeResult {
    let mut stream = TcpStream::connect(&host).unwrap();
    let req_bytes = req
        .write_to_bytes()
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    packet::write(&mut stream, HT_COMPUTE_REQUEST, &req_bytes).unwrap();
    let (header, res_bytes) = &packet::read(&mut stream, 1).unwrap()[0];
    assert_eq!(header.ty, HT_COMPUTE_RESULT);
    protobuf::parse_from_bytes::<protos::ComputeResult>(&res_bytes)
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap()
}

/*****************************************************************************
 * Service API
 *****************************************************************************/
/// Notify controller about compute request start.
pub fn notify_start(controller_addr: &str, service_id: u32, description: String) {
    let mut stream = TcpStream::connect(&controller_addr).unwrap();
    let mut req = protos::NotifyStart::default();
    req.set_service_name(format!("KarlService-{}", service_id));
    req.set_description(description);
    let req_bytes = req
        .write_to_bytes()
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    packet::write(&mut stream, HT_NOTIFY_START, &req_bytes).unwrap();
}

/// Notify controller about compute request end.
pub fn notify_end(controller_addr: &str, service_id: u32) {
    let mut stream = TcpStream::connect(&controller_addr).unwrap();
    let mut req = protos::NotifyEnd::default();
    req.set_service_name(format!("KarlService-{}", service_id));
    let req_bytes = req
        .write_to_bytes()
        .map_err(|e| Error::SerializationError(format!("{:?}", e)))
        .unwrap();
    packet::write(&mut stream, HT_NOTIFY_END, &req_bytes).unwrap();
}
