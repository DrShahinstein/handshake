use crate::{Result, net};
use std::net::UdpSocket;

pub fn start(target: &str) -> Result<()> {
  let socket = UdpSocket::bind("0.0.0.0:0")?;
  socket.connect(target)?;
  socket.send(b"HSH_SYN")?;

  let mut buf = [0u8; 256];
  let (bytes, src) = socket.recv_from(&mut buf)?;

  if &buf[..bytes] == b"HSH_ACK" {
    net::session::start(socket, src)?;
  }

  Ok(())
}

