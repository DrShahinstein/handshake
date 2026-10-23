use std::io;
use std::net::{UdpSocket, SocketAddr};
use crate::net::{self, Frame, FrameType};
use crate::Result;

pub fn start(socket: &UdpSocket) -> Result<SocketAddr> {
  loop {
    let (f, src) = net::recv_frame(socket)?;

    if f.frame_type == FrameType::Syn {
      let ack = Frame::ack(f.seq_id);
      net::send_frame_to(socket, &ack, src)?;
      return Ok(src);
    }
  }
}

pub fn _0_0_0_0(port: u16) -> io::Result<UdpSocket> {
  UdpSocket::bind(("0.0.0.0", port))
}

// udp routing table lookup trick
pub fn get_lan_ip() -> Option<String> {
  let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
  socket.connect("8.8.8.8:80").ok()?;
  socket
    .local_addr()
    .ok()
    .map(|addr| addr.ip().to_string())
}

