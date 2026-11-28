use std::thread;
use std::time::Duration;
use crate::{Result, net::session::Session};

const UI_TICK: Duration = Duration::from_millis(50);

pub fn start(session: Session) -> Result<()> {
  println!("\nTODO: ui with peer {}", session.peer_addr());

  loop {
    let _ = session.poll()?;
    thread::sleep(UI_TICK);
  }
}

