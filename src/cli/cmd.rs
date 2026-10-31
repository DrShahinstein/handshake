use std::io::{self, Write};
use std::net::UdpSocket;
use crate::{
  HshError, Result, cli,
  net::{self, candidate::Candidates},
};

pub fn punch(port: u16, target: Option<&str>) -> Result<()> {
  let socket = UdpSocket::bind(("0.0.0.0", port))?;
  let port   = socket.local_addr()?.port();

  println!("> bound to 0.0.0.0:{port}");

  let candidates = Candidates::gather(&socket, port);
  candidates.show();

  let input = match target {
    Some(target) => target.to_string(),
    None         => read_peer_link()?,
  };

  let peers = Candidates::resolve_target(&input)?;
  let peer  = net::punch::start(&socket, &peers)?;

  net::session::start(socket, peer)?;

  Ok(())
}

pub fn help() -> Result<()> {
  print!("{}", cli::HELP_MSG);
  Ok(())
}

pub fn version() -> Result<()> {
  let version = env!("CARGO_PKG_VERSION");
  println!("v{}", version);
  Ok(())
}

/* - - - */

/* internal helper */
fn read_peer_link() -> Result<String> {
  print!("> paste your peer's link: ");
  io::stdout().flush()?;

  let mut link = String::new();
  io::stdin().read_line(&mut link)?;

  let link = link.trim().to_string();
  if link.is_empty() {
    return Err(HshError::InvalidLink);
  }

  Ok(link)
}

