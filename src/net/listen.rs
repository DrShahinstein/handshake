use std::io;
use std::net::{UdpSocket, SocketAddr};

pub fn start(socket: &UdpSocket) -> io::Result<SocketAddr> {
  let mut buf = [0u8; 256];

  loop {
    let (bytes, src) = socket.recv_from(&mut buf)?;
    if bytes < 7 {
      continue;
    }

    let payload = &buf[..bytes];

    if &payload[0..7] != b"HSH_SYN" {
      socket.send_to(b"i don't speak this language", src)?;
      continue;
    }

    socket.send_to(b"HSH_ACK", src)?;

    break Ok(src);
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

