/* STUN HEADER (20B)
 *  +--------------------+--------------------+----------------------+-----------------------+
 *  |  Message Type (2B) | Message Length(2B) |   Magic Cookie (4B)  |  Transaction ID (12B) |
 *  +--------------------+--------------------+----------------------+-----------------------+
 * 
 * XOR-MAPPED-ADDRESS ATTRIBUTE (Type: 0x0020)
 *  +--------------------+--------------------+---------------+---------------+---------------+---------------+
 *  |   Attr Type (2B)   |  Attr Length (2B)  | Reserved (1B) |  Family (1B)  | XOR-Port (2B) | XOR-IPv4 (4B) |
 *  +--------------------+--------------------+---------------+---------------+---------------+---------------+
 */

use std::net::{UdpSocket, SocketAddr, ToSocketAddrs, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::io::{Error, ErrorKind};
use crate::Result;

// see: RFC 5389
const MAGIC_COOKIE: u32 = 0x2112_A442;

const STUN_SERVERS: &[&str] = &[
  "stun.l.google.com:19302",
  "stun1.l.google.com:19302",
  "stun2.l.google.com:19302",
  "stun.cloudflare.com:3478",
];

pub fn discover(socket: &UdpSocket) -> Result<SocketAddr> {
  let original_timeout = socket.read_timeout().ok();
  socket.set_read_timeout(Some(Duration::from_secs(2)))?;

  // preparing 20 byte stun header
  let mut req = [0u8; 20];
  req[0..2].copy_from_slice(&0x0001u16.to_be_bytes());     // Binding Request -> tell me what my nat binding is
  req[2..4].copy_from_slice(&0x0000u16.to_be_bytes());     // Length (0)
  req[4..8].copy_from_slice(&MAGIC_COOKIE.to_be_bytes());  // Magic Cookie
  req[8..20].copy_from_slice(b"HSH_STUN_REQ");             // Transaction ID

  let mut buf = [0u8; 512];

  for server in STUN_SERVERS {
    let addr = match server.to_socket_addrs() {
      Ok(mut addrs) => match addrs.next() {
        Some(a) => a,
        None => continue,
      },
      Err(_) => continue,
    };

    if socket.send_to(&req, addr).is_err() {
      continue;
    }

    if let Ok((bytes, _)) = socket.recv_from(&mut buf) {
      if let Some(public_addr) = parse_stun_response(&buf[..bytes]) {
        let _ = socket.set_read_timeout(original_timeout.flatten());
        return Ok(public_addr);
      }
    }
  }

  let _ = socket.set_read_timeout(original_timeout.flatten());

  Err(Error::new(ErrorKind::TimedOut, "All STUN servers failed to respond").into())
}

fn parse_stun_response(buf: &[u8]) -> Option<SocketAddr> {
  if buf.len() < 20 {
    return None;
  }

  let msg_type = u16::from_be_bytes(buf[0..2].try_into().ok()?);
  if msg_type != 0x0101 {
    return None;
  }

  let msg_len = u16::from_be_bytes(buf[2..4].try_into().ok()?) as usize;
  if buf.len() < 20 + msg_len {
    return None;
  }

  let mut offset = 20;
  while (offset+4) <= (20+msg_len) {
    let attr_type = u16::from_be_bytes(buf[offset..offset + 2].try_into().ok()?);
    let attr_len  = u16::from_be_bytes(buf[offset + 2..offset + 4].try_into().ok()?) as usize;

    offset += 4;

    if offset+attr_len > buf.len() {
      return None;
    }

    if attr_type == 0x0020 && attr_len >= 8 {
      let attr_data = &buf[offset..offset + attr_len];
      let family    = attr_data[1];

      // IPv4
      if family == 0x01 { 
        let xor_port = u16::from_be_bytes(attr_data[2..4].try_into().ok()?);
        let port     = xor_port ^ (MAGIC_COOKIE >> 16) as u16;
        let xor_ip   = u32::from_be_bytes(attr_data[4..8].try_into().ok()?);
        let ip_u32   = xor_ip ^ MAGIC_COOKIE;
        let ip       = Ipv4Addr::from(ip_u32);
        return Some(SocketAddr::new(IpAddr::V4(ip), port));
      }
    }

    let padding = (4 - (attr_len % 4)) % 4;
    offset += attr_len + padding;
  }

  None
}

