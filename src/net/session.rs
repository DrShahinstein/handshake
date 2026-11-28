use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use crate::{net, HshError, Result};

const PING_EVERY: Duration = Duration::from_secs(5);
const MISSED_PINGS: u8     = 3;

#[derive(Debug)]
enum SessionEvent {
  Frame(net::Frame),
  PeerSilent,
  Failed(HshError),
}

pub struct Session {
  socket: UdpSocket,
  peer_addr: SocketAddr,
  events: Receiver<SessionEvent>,
}

impl Session {
  pub fn start(socket: UdpSocket, peer_addr: SocketAddr) -> Result<Self> {
    let reader   = socket.try_clone()?;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      if let Err(err) = listen(reader, peer_addr, &tx) {
        let _ = tx.send(SessionEvent::Failed(err));
      }
    });

    Ok(Self { socket, peer_addr, events: rx })
  }

  pub fn send(&self, frame: &net::Frame) -> Result<()> {
    net::send_frame_to(&self.socket, frame, self.peer_addr)
  }

  pub fn poll(&self) -> Result<Option<net::Frame>> {
    match self.events.try_recv() {
      Ok(SessionEvent::Frame(frame))        => Ok(Some(frame)),
      Ok(SessionEvent::PeerSilent)          => Err(HshError::PeerSilent),
      Ok(SessionEvent::Failed(err))         => Err(err),
      Err(mpsc::TryRecvError::Empty)        => Ok(None),
      Err(mpsc::TryRecvError::Disconnected) => Err(HshError::PeerSilent),
    }
  }

  pub fn peer_addr(&self) -> SocketAddr {
    self.peer_addr
  }
}

fn listen(socket: UdpSocket, peer_addr: SocketAddr, tx: &Sender<SessionEvent>) -> Result<()> {
  socket.set_read_timeout(Some(PING_EVERY))?;

  let mut missed = 0;

  loop {
    match net::recv_frame_or_timeout(&socket)? {
      Some((frame, src)) if src == peer_addr => {
        missed = 0;

        match frame.frame_type {
          net::FrameType::Ping => net::send_frame_to(&socket, &net::Frame::pong(), peer_addr)?,
          net::FrameType::Pong => {}
          _ => {
            if tx.send(SessionEvent::Frame(frame)).is_err() {
              return Ok(());
            }
          }
        }
      }
      Some(_) => {} // stray packet from another address
      None => {
        missed += 1;

        if missed >= MISSED_PINGS {
          let _ = tx.send(SessionEvent::PeerSilent);
          return Ok(());
        }

        net::send_frame_to(&socket, &net::Frame::ping(), peer_addr)?;
      }
    }
  }
}

