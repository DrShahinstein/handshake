use std::io;
use std::net::UdpSocket;

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

