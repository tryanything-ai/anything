#![allow(unused)]
use rand::prelude::*;

use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs, UdpSocket,
};

pub type Port = u16;

pub async fn get_unused_port() -> Option<u16> {
    let mut rng = rand::thread_rng();

    // Try random port first
    for _ in 0..10 {
        let port = rng.gen_range(15000..25000);
        if is_free(port) {
            return Some(port);
        }
    }

    // Ask the OS for a port
    for _ in 0..10 {
        if let Some(port) = ask_free_tcp_port() {
            // Test that the udp port is free as well
            if is_free_udp(port) {
                return Some(port);
            }
        }
    }

    // Give up
    None
}

pub fn is_free_tcp(port: Port) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_tcp(ipv6).is_some() && test_bind_tcp(ipv4).is_some()
}

/// Check if a port is free on UDP
pub fn is_free_udp(port: Port) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_udp(ipv6).is_some() && test_bind_udp(ipv4).is_some()
}

// Try to bind to a socket using UDP
fn test_bind_udp<A: ToSocketAddrs>(addr: A) -> Option<Port> {
    Some(UdpSocket::bind(addr).ok()?.local_addr().ok()?.port())
}

// Try to bind to a socket using TCP
fn test_bind_tcp<A: ToSocketAddrs>(addr: A) -> Option<Port> {
    Some(TcpListener::bind(addr).ok()?.local_addr().ok()?.port())
}

/// Asks the OS for a free port
fn ask_free_tcp_port() -> Option<Port> {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);

    test_bind_tcp(ipv6).or_else(|| test_bind_tcp(ipv4))
}

/// Check if a port is free on both TCP and UDP
pub fn is_free(port: Port) -> bool {
    is_free_tcp(port) && is_free_udp(port)
}
