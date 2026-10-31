use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
use crate::net::{self, Frame, FrameType, INITIAL_SEQ_ID, Status};
use crate::{HshError, Result};

const RETRY_INTERVAL: Duration = Duration::from_millis(250);
const CONFIRM_GRACE: Duration  = Duration::from_millis(500);
const MAX_ATTEMPTS: usize      = 20;

struct PunchState {
  addr: Option<SocketAddr>,
  syn_acked: bool,
  confirmed_at: Option<Instant>,
  last_send_error: Option<HshError>,
}

pub fn start(socket: &UdpSocket, peer_addrs: &[SocketAddr]) -> Result<SocketAddr> {
  if peer_addrs.is_empty() {
    return Err(HshError::InvalidLink);
  }

  let original_timeout = socket.read_timeout().ok().flatten();
  socket.set_read_timeout(Some(RETRY_INTERVAL))?;

  let result = punch_loop(socket, peer_addrs);
  socket.set_read_timeout(original_timeout)?;

  result
}

fn punch_loop(socket: &UdpSocket, peer_addrs: &[SocketAddr]) -> Result<SocketAddr> {
  for addr in peer_addrs {
    println!("...punching {addr}");
  }

  let syn       = Frame::syn();
  let mut state = PunchState::new();

  for _ in 0..MAX_ATTEMPTS {
    if state.confirmed_at.is_none() {
      state.send_syn(socket, peer_addrs, &syn);
    }

    while let Some((frame, src)) = recv_frame_or_timeout(socket)? {
      state.handle_frame(socket, frame, src);
    }

    if let Some(addr) = state.confirmed_addr() {
      return Ok(addr);
    }
  }

  state.into_result()
}

impl PunchState {
  fn new() -> Self {
    Self {
      addr: None,
      syn_acked: false,
      confirmed_at: None,
      last_send_error: None,
    }
  }

  fn send_syn(&mut self, socket: &UdpSocket, peer_addrs: &[SocketAddr], syn: &Frame) {
    for addr in peer_addrs {
      self.record_send(net::send_frame_to(socket, syn, *addr));
    }
  }

  fn handle_frame(&mut self, socket: &UdpSocket, frame: Frame, src: SocketAddr) {
    if frame.status != Status::OK {
      return;
    }

    match frame.frame_type {
      FrameType::Syn => self.handle_syn(socket, frame, src),
      FrameType::Ack => self.handle_ack(frame),
    }
  }

  fn handle_syn(&mut self, socket: &UdpSocket, frame: Frame, src: SocketAddr) {
    let ack = Frame::ack(frame.seq_id);

    self.record_send(net::send_frame_to(socket, &ack, src));

    if self.addr.is_none() {
      println!("...peer answered from {src}");
      self.addr = Some(src);
    }

    self.confirm_if_ready();
  }

  fn handle_ack(&mut self, frame: Frame) {
    if frame.seq_id == INITIAL_SEQ_ID {
      self.syn_acked = true;
      self.confirm_if_ready();
    }
  }

  fn confirm_if_ready(&mut self) {
    if self.confirmed_at.is_none() && self.addr.is_some() && self.syn_acked {
      self.confirmed_at = Some(Instant::now());
    }
  }

  fn confirmed_addr(&self) -> Option<SocketAddr> {
    let confirmed_at = self.confirmed_at?;

    if confirmed_at.elapsed() >= CONFIRM_GRACE {
      self.addr
    } else {
      None
    }
  }

  fn record_send(&mut self, result: Result<()>) {
    match result {
      Ok(())   => self.last_send_error = None,
      Err(err) => self.last_send_error = Some(err),
    }
  }

  fn into_result(self) -> Result<SocketAddr> {
    if let (Some(addr), Some(_)) = (self.addr, self.confirmed_at) {
      return Ok(addr);
    }

    if let Some(err) = self.last_send_error {
      return Err(err);
    }

    Err(HshError::ExpectedAck)
  }
} // impl end

/* - - - */

fn recv_frame_or_timeout(socket: &UdpSocket) -> Result<Option<(Frame, SocketAddr)>> {
  match net::recv_frame(socket) {
    Ok(frame) => Ok(Some(frame)),
    Err(HshError::IoErr(err)) if is_timeout(&err) => Ok(None),
    Err(err) => Err(err),
  }
}

fn is_timeout(err: &Error) -> bool {
  matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut)
}

