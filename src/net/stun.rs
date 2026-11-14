/* STUN (RFC 5389) — only as much of it as we need:
 * ask a server how it sees us, read back the mapped endpoint.
 *
 * Header (20B), in the request and in the response:
 *  +--------------------+--------------------+----------------------+-----------------------+
 *  |  Message Type (2B) | Message Length(2B) |   Magic Cookie (4B)  |  Transaction ID (12B) |
 *  +--------------------+--------------------+----------------------+-----------------------+
 *
 * XOR-MAPPED-ADDRESS ATTRIBUTE (Type: 0x0020)
 *  +--------------------+--------------------+---------------+---------------+---------------+---------------+
 *  |   Attr Type (2B)   |  Attr Length (2B)  | Reserved (1B) |  Family (1B)  | XOR-Port (2B) | XOR-IPv4 (4B) |
 *  +--------------------+--------------------+---------------+---------------+---------------+---------------+
 *
 * port = XOR-Port ^ (Magic Cookie >> 16)
 * ip   = XOR-IPv4 ^ Magic Cookie
 *
 * Note: The binding exchange is identical in RFC 5389 and RFC 8489 (which obsoletes it), so either answers us.
 *       Servers older than 5389 would not: they lack the magic cookie and send MAPPED-ADDRESS instead.
 */

use std::net::{UdpSocket, SocketAddr, ToSocketAddrs, IpAddr, Ipv4Addr};
use std::ops::Range;
use std::time::Duration;
use crate::{HshError, Result};

// see: RFC 5389
const MAGIC_COOKIE:       u32 = 0x2112_A442;
const BINDING_REQUEST:    u16 = 0x0001;
const BINDING_SUCCESS:    u16 = 0x0101;
const XOR_MAPPED_ADDRESS: u16 = 0x0020;
const FAMILY_IPV4:        u8  = 0x01;

const HEADER_LEN:      usize = 20;
const TX_ID_LEN:       usize = 12;
const ATTR_HEADER_LEN: usize = 4;

const RESPONSE_TIMEOUT: Duration = Duration::from_secs(2);

const TYPE_AT:   Range<usize> = 0..2;
const LENGTH_AT: Range<usize> = 2..4;
const COOKIE_AT: Range<usize> = 4..8;
const TX_ID_AT:  Range<usize> = 8..20;

const STUN_SERVERS: &[&str] = &[
  "stun.l.google.com:19302",
  "stun1.l.google.com:19302",
  "stun2.l.google.com:19302",
  "stun.cloudflare.com:3478",
];

pub fn discover(socket: &UdpSocket) -> Result<SocketAddr> {
  let original_timeout = socket.read_timeout().ok().flatten();
  socket.set_read_timeout(Some(RESPONSE_TIMEOUT))?;

  let result = query_servers(socket);

  let _ = socket.set_read_timeout(original_timeout);

  result
}

/* - - - */

fn query_servers(socket: &UdpSocket) -> Result<SocketAddr> {
  for server in STUN_SERVERS {
    let addr = match resolve(server) {
      Some(addr) => addr,
      None       => continue,
    };

    // fresh transaction per request: RFC 5389 section 6
    let tx_id = transaction_id()?;

    if let Some(public) = exchange(socket, addr, &tx_id) {
      return Ok(public);
    }
  }

  Err(HshError::StunNoReply)
}

fn exchange(socket: &UdpSocket, addr: SocketAddr, tx_id: &[u8; TX_ID_LEN]) -> Option<SocketAddr> {
  let mut buf = [0u8; 512];

  socket.send_to(&request(tx_id), addr).ok()?;

  while let Ok((bytes, src)) = socket.recv_from(&mut buf) {
    if src != addr {
      continue;
    }

    if let Some(public) = parse_response(&buf[..bytes], tx_id) {
      return Some(public);
    }
  }

  None
}

fn resolve(server: &str) -> Option<SocketAddr> {
  server.to_socket_addrs().ok()?.next()
}

fn transaction_id() -> Result<[u8; TX_ID_LEN]> {
  let mut tx_id = [0u8; TX_ID_LEN];
  getrandom::fill(&mut tx_id).map_err(|_| HshError::StunEntropy)?;

  Ok(tx_id)
}

fn request(tx_id: &[u8; TX_ID_LEN]) -> [u8; HEADER_LEN] {
  let mut req = [0u8; HEADER_LEN];

  req[TYPE_AT].copy_from_slice(&BINDING_REQUEST.to_be_bytes()); // tell me what my nat binding (public endpoint) is
  // LENGTH_AT stays zero: a binding request carries no attributes
  req[COOKIE_AT].copy_from_slice(&MAGIC_COOKIE.to_be_bytes());
  req[TX_ID_AT].copy_from_slice(tx_id);

  req
}

fn parse_response(buf: &[u8], tx_id: &[u8; TX_ID_LEN]) -> Option<SocketAddr> {
  if buf.len() < HEADER_LEN {
    return None;
  }
  if read_u16(buf, TYPE_AT)? != BINDING_SUCCESS {
    return None;
  }
  if read_u32(buf, COOKIE_AT)? != MAGIC_COOKIE {
    return None;
  }

  // only a reply to this exact request carries our transaction id
  if buf[TX_ID_AT] != tx_id[..] {
    return None;
  }

  let body_len = read_u16(buf, LENGTH_AT)? as usize;
  let body     = buf.get(HEADER_LEN..HEADER_LEN + body_len)?;

  for (attr_type, data) in attributes(body) {
    if attr_type == XOR_MAPPED_ADDRESS {
      return xor_mapped_address(data);
    }
  }

  None
}

// walks the attribute list, hiding the type-length-value and padding work
struct Attributes<'a> {
  body:   &'a [u8],
  offset: usize,
}

fn attributes(body: &[u8]) -> Attributes<'_> {
  Attributes {body, offset: 0}
}

impl<'a> Iterator for Attributes<'a> {
  type Item = (u16, &'a [u8]);

  fn next(&mut self) -> Option<Self::Item> {
    let header    = self.body.get(self.offset..self.offset + ATTR_HEADER_LEN)?;
    let attr_type = read_u16(header, 0..2)?;
    let attr_len  = read_u16(header, 2..4)? as usize;
    let start     = self.offset + ATTR_HEADER_LEN;
    let data      = self.body.get(start..start + attr_len)?;

    self.offset = start+attr_len+padding(attr_len);

    Some((attr_type, data))
  }
}

fn xor_mapped_address(data: &[u8]) -> Option<SocketAddr> {
  // 1B reserved | 1B family | 2B xor-port | 4B xor-ip
  if data.len() < 8 || data[1] != FAMILY_IPV4 {
    return None;
  }

  let port = read_u16(data, 2..4)? ^ (MAGIC_COOKIE >> 16) as u16;
  let ip   = Ipv4Addr::from(read_u32(data, 4..8)? ^ MAGIC_COOKIE);

  Some(SocketAddr::new(IpAddr::V4(ip), port))
}

/* - - - */

fn read_u16(buf: &[u8], at: Range<usize>) -> Option<u16> {
  Some(u16::from_be_bytes(buf.get(at)?.try_into().ok()?))
}

fn read_u32(buf: &[u8], at: Range<usize>) -> Option<u32> {
  Some(u32::from_be_bytes(buf.get(at)?.try_into().ok()?))
}

// attributes are padded to a 4 byte boundary
fn padding(len: usize) -> usize {
  (4 - (len % 4)) % 4
}

