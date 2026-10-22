use std::net::UdpSocket;
use std::io;

pub fn start(target: &str) -> io::Result<()> {
  let socket = UdpSocket::bind("0.0.0.0:0")?;
  socket.connect(target)?;
  socket.send(&[0x48, 0x53, 0x48])?; // HSH
  Ok(())
}

