/* PROTOCOL
*  +------------------+------------------+------------------+----------------------+--------------------+--------------------+
*  | Magic Bytes (3B) | Frame Type (1B)  | Status Code (1B) | Sequence/Req ID (4B) | Payload Length(4B) | Payload (Variable) |
*  +------------------+------------------+------------------+----------------------+--------------------+--------------------+
*/

pub mod listen;
pub mod connect;
pub mod session;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use derive_new::new;
use crate::Result;
use std::net::{UdpSocket, SocketAddr};

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,IntoPrimitive,TryFromPrimitive)]
pub enum Status {
  OK=200,
  ERR=201,
}

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,IntoPrimitive,TryFromPrimitive)]
pub enum FrameType {
  Syn,
  Ack,
}

#[derive(new)]
pub struct Frame {
  pub ftype: FrameType,
  pub status: Status,
  pub seq_id: u32,
  pub len: u32,
  pub payload: Vec<u8>
}

impl Frame {
  pub const MAGIC_BYTES: &[u8;3] = b"HSH";
  pub const HEADER_SIZE: usize   = 13;

  pub fn to_bytes(&self) -> Vec<u8> {
    let payload_len = self.payload.len() as u32;
    let mut bytes   = Vec::with_capacity(Self::HEADER_SIZE + self.payload.len());

    bytes.extend_from_slice(Self::MAGIC_BYTES);             // 3B: Magic Bytes
    bytes.push(self.ftype.into());                          // 1B: Frame Type
    bytes.push(self.status.into());                         // 1B: Status Code
    bytes.extend_from_slice(&self.seq_id.to_be_bytes());    // 4B: Sequence ID
    bytes.extend_from_slice(&payload_len.to_be_bytes());    // 4B: Payload Length
    bytes.extend_from_slice(&self.payload);                 // Var: Payload

    bytes
  }
}

pub fn send_frame(socket: &UdpSocket, frame: &Frame) -> Result<()> {
  let bytes = frame.to_bytes();
  socket.send(&bytes)?;
  Ok(())
}

pub fn send_frame_to(s: &UdpSocket, f: &Frame, to: &SocketAddr) -> Result<()> {
  let b = f.to_bytes();
  s.send_to(&b, to)?;
  Ok(())
}

pub fn recv_frame(_socket: &UdpSocket) -> Result<()> {
  unimplemented!()
}

