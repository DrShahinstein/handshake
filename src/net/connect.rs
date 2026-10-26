use crate::{Result, HshError};
use crate::net::{self, Frame, FrameType};
use std::net::UdpSocket;

pub fn start(target: &str) -> Result<()> {
  let socket = UdpSocket::bind("0.0.0.0:0")?;
  socket.connect(target)?;

  let syn = Frame::syn();
  net::send_frame(&socket, &syn)?;

  let (f, src) = net::recv_frame(&socket)?;

  if f.frame_type != FrameType::Ack {
    return Err(HshError::ExpectedAck);
  }

  net::session::start(socket, src)?;

  Ok(())
}

