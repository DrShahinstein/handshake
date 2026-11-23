use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration};
use derive_new::new;
use crate::{net, HshError, Result};

const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(5);
const PEER_TIMEOUT:       Duration = Duration::from_secs(15);

#[derive(new)]
pub struct SessionCtx {
  socket: UdpSocket,
  peer_addr: SocketAddr,
}

impl SessionCtx {
  pub fn start(&self) -> Result<Receiver<HshError>> {
    println!("\n> starting session with peer at {}", self.peer_addr);
    self.keepalive()
  }

  fn keepalive(&self) -> Result<Receiver<HshError>> {
    self.socket.set_read_timeout(Some(PEER_TIMEOUT))?;

    let (tx,rx) = mpsc::channel();

    let socket1 = Self::new(self.socket.try_clone()?, self.peer_addr);
    let socket2 = Self::new(self.socket.try_clone()?, self.peer_addr);

    thread::spawn(move || {
      if let Err(e) = socket1.ping() {
        eprintln!("{e}");
      }
    });

    thread::spawn(move || {
      if let Err(e) = socket2.pong() {
        let _ = tx.send(e);
      }
    });

    Ok(rx)
  }

  fn ping(&self) -> Result<()> {
    loop {
      thread::sleep(KEEPALIVE_INTERVAL);
      net::send_frame_to(&self.socket, &net::Frame::ping(), self.peer_addr)?;
    }
  }

  fn pong(&self) -> Result<()> {
    loop {
      let Some((frame, src)) = net::recv_frame_or_timeout(&self.socket)? else {
        return Err(HshError::PeerSilent);
      };

      if frame.frame_type == net::FrameType::Ping {
        net::send_frame_to(&self.socket, &net::Frame::pong(), src)?;
      }
    }
  }
}
