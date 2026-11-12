use std::io::{self, Write};
use std::net::{SocketAddr, UdpSocket};
use crate::{
  HshError, Result, cli,
  net::{self, candidate::Candidates},
};

pub fn punch(port: u16, target: Option<&str>) -> Result<()> {
  let socket = UdpSocket::bind(("0.0.0.0", port))?;
  let port   = socket.local_addr()?.port();

  println!("> bound to 0.0.0.0:{port}");

  let mut candidates = Candidates::gather(&socket, port);
  show(&candidates);

  let input = match target {
    Some(target) => target.to_string(),
    None         => read_peer()?,
  };

  if let Some((announced, current)) = candidates.recheck_public(&socket) {
    show_drift(announced, current, &candidates);
  }

  let peer_addrs = Candidates::resolve_target(&input)?;
  let peer_addr  = net::punch::start(&socket, &peer_addrs)?;

  net::session::start(socket, peer_addr)?;

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

fn read_peer() -> Result<String> {
  print!(">> paste your peer's handshake link or endpoint: ");
  io::stdout().flush()?;

  let mut link = String::new();
  io::stdin().read_line(&mut link)?;

  let link = link.trim().to_string();
  if link.is_empty() {
    return Err(HshError::InvalidLink);
  }

  Ok(link)
}

fn show(cnds: &Candidates) {
  println!("  * loopback: {}", cnds.loopback);

  if let Some(lan) = cnds.lan {
    println!("  * lan: {lan}");
  } else {
    println!("  ! lan: offline");
  }

  if let Some(public) = cnds.public {
    println!("> discovered public nat endpoint: \x1b[1;32m{public}\x1b[0m");
  } else {
    println!("> nat discovery failed or skipped.");
  }

  let raw_link    = cnds.to_base64();
  let styled_link = format!("\x1b[4;36m{raw_link}\x1b[0m");

  println!("> share this link with your peer:\n{styled_link}\n");
}

fn show_drift(announced: SocketAddr, current: SocketAddr, cnds: &Candidates) {
  println!();
  println!("! public endpoint moved: \x1b[90m{announced}\x1b[0m -> \x1b[1;32m{current}\x1b[0m");
  println!("  your shared link is now stale; this attempt may fail.");

  let raw_link    = cnds.to_base64();
  let styled_link = format!("\x1b[4;36m{raw_link}\x1b[0m");

  println!("> fresh link:\n{styled_link}\n");
}

