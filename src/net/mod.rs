/* PROTOCOL
*  +------------------+------------------+------------------+----------------------+--------------------+--------------------+
*  | Magic Bytes (3B) | Frame Type (1B)  | Status Code (1B) | Sequence/Req ID (4B) | Payload Length(4B) | Payload (Variable) |
*  +------------------+------------------+------------------+----------------------+--------------------+--------------------+
*/

pub mod listen;
pub mod connect;
pub mod session;
pub mod stun;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use derive_new::new;
use crate::Result;
use std::net::{UdpSocket, SocketAddr};

pub const INITIAL_SEQ_ID: u32 = 1;

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,IntoPrimitive,TryFromPrimitive)]
pub enum Status {
  OK=200,
  ERR=201,
}

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,IntoPrimitive,TryFromPrimitive)]
pub enum FrameType {
  Syn=1,
  Ack=2,
}

#[derive(new)]
pub struct Frame {
  pub frame_type: FrameType,
  pub status: Status,
  pub seq_id: u32,
  pub payload: Vec<u8>
}

impl Frame {
  pub const MAGIC_BYTES: &[u8;3] = b"HSH";
  pub const HEADER_SIZE: usize   = 13;

  pub fn syn() -> Self {
    Self::new(
      FrameType::Syn,
      Status::OK,
      INITIAL_SEQ_ID,
      vec![],
    )
  }

  pub fn ack(seq_id: u32) -> Self {
    Self::new(FrameType::Ack, Status::OK, seq_id, vec![])
  }

  pub fn payload_len(&self) -> u32 {
    self.payload.len() as u32
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let header_len  = Self::HEADER_SIZE;
    let payload_len = self.payload_len();
    let total_len   = header_len + (payload_len as usize);

    let mut bytes = Vec::with_capacity(total_len);

    bytes.extend_from_slice(Self::MAGIC_BYTES);             // 3B: Magic Bytes (HSH)
    bytes.push(self.frame_type.into());                     // 1B: Frame Type
    bytes.push(self.status.into());                         // 1B: Status Code
    bytes.extend_from_slice(&self.seq_id.to_be_bytes());    // 4B: Sequence ID
    bytes.extend_from_slice(&payload_len.to_be_bytes());    // 4B: Payload Length
    bytes.extend_from_slice(&self.payload);                 // Var: Payload

    bytes
  }

  pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
    let header_len = Self::HEADER_SIZE;
    let bytes_len  = bytes.len();

    if bytes_len < header_len {
      return None;
    }
    if &bytes[0..3] != Self::MAGIC_BYTES {
      return None;
    }

    let frame_type  = FrameType::try_from(bytes[3]).ok()?;
    let status      = Status::try_from(bytes[4]).ok()?;
    let seq_id      = u32::from_be_bytes(bytes[5..9].try_into().ok()?);
    let payload_len = u32::from_be_bytes(bytes[9..13].try_into().ok()?);
    let total_len   = header_len + (payload_len as usize);

    if bytes_len < total_len {
      return None;
    }

    let payload = bytes[header_len..total_len].to_vec();

    Some(Self {
      frame_type,
      status,
      seq_id,
      payload,
    })
  }
}

pub fn send_frame(socket: &UdpSocket, frame: &Frame) -> Result<()> {
  let bytes = frame.to_bytes();
  socket.send(&bytes)?;
  Ok(())
}

pub fn send_frame_to(s: &UdpSocket, f: &Frame, to: SocketAddr) -> Result<()> {
  let b = f.to_bytes();
  s.send_to(&b, to)?;
  Ok(())
}

pub fn recv_frame(socket: &UdpSocket) -> Result<(Frame, SocketAddr)> {
  let mut buf = [0u8; 2048];

  loop {
    let (bytes, src)   = socket.recv_from(&mut buf)?;
    let incoming_frame = &buf[..bytes];
    
    if let Some(frame) = Frame::from_bytes(&incoming_frame) {
      return Ok((frame, src));
    }
  }
}

