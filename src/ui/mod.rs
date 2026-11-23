use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use crate::{HshError, Result};

pub fn start(peer: SocketAddr, events: Receiver<HshError>) -> Result<()> {
  println!("\nTODO: ui with peer {peer}");

  match events.recv() {
    Ok(e)  => Err(e),
    Err(_) => Ok(()),
  }
}
