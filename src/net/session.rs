use std::net::{UdpSocket, SocketAddr};
use crate::Result;

pub fn start(_socket: UdpSocket, peer: SocketAddr) -> Result<()> {
  println!("Session: handshaked with {peer}");
  Ok(())
}

