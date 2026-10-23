use crate::Result;
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};

pub fn discover(_socket: &UdpSocket) -> Result<SocketAddr> {
  Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(6,9,6,9)), 6969))
}

