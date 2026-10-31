use std::io::{Error, ErrorKind};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
use crate::net::{self, Frame, FrameType, INITIAL_SEQ_ID, Status};
use crate::{HshError, Result};

const RETRY_INTERVAL: Duration = Duration::from_millis(250);
const CONFIRM_GRACE: Duration  = Duration::from_millis(500);
const MAX_ATTEMPTS: usize      = 20;

struct PunchState {
  peer: Option<SocketAddr>,
  syn_acked: bool,
  confirmed_at: Option<Instant>,
  last_send_error: Option<HshError>,
}

pub fn start(socket: &UdpSocket, peers: &[SocketAddr]) -> Result<SocketAddr> {
  if peers.is_empty() {
    return Err(HshError::InvalidLink);
  }

  let original_timeout = socket.read_timeout().ok().flatten();
  socket.set_read_timeout(Some(RETRY_INTERVAL))?;

  let result = punch_loop(socket, peers);
  socket.set_read_timeout(original_timeout)?;

  result
}

fn punch_loop(socket: &UdpSocket, peers: &[SocketAddr]) -> Result<SocketAddr> {
  for peer in peers {
    println!("...punching {peer}");
  }

  let syn       = Frame::syn();
  let mut state = PunchState::new();

  for _ in 0..MAX_ATTEMPTS {
    if state.confirmed_at.is_none() {
      state.send_syn(socket, peers, &syn);
    }

    while let Some((frame, src)) = recv_frame_or_timeout(socket)? {
      state.handle_frame(socket, frame, src);
    }

    if let Some(peer) = state.confirmed_peer() {
      return Ok(peer);
    }
  }

  state.into_result()
}

impl PunchState {
  fn new() -> Self {
    Self {
      peer: None,
      syn_acked: false,
      confirmed_at: None,
      last_send_error: None,
    }
  }

  fn send_syn(&mut self, socket: &UdpSocket, peers: &[SocketAddr], syn: &Frame) {
    for peer in peers {
      self.record_send(net::send_frame_to(socket, syn, *peer));
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

    if self.peer.is_none() {
      println!("...peer answered from {src}");
      self.peer = Some(src);
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
    if self.confirmed_at.is_none() && self.peer.is_some() && self.syn_acked {
      self.confirmed_at = Some(Instant::now());
    }
  }

  fn confirmed_peer(&self) -> Option<SocketAddr> {
    let confirmed_at = self.confirmed_at?;

    if confirmed_at.elapsed() >= CONFIRM_GRACE {
      self.peer
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
    if let (Some(peer), Some(_)) = (self.peer, self.confirmed_at) {
      return Ok(peer);
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

