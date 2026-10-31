use crate::Result;
use std::net::{SocketAddr, UdpSocket};

pub fn start(_socket: UdpSocket, peer: SocketAddr) -> Result<()> {
  println!("...connected to {peer}");
  Ok(())
}

