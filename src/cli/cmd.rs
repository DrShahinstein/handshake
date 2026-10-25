use crate::{Result, cli, net::{self, candidate::Candidates}};

pub fn listen(port: u16) -> Result<()> {
  let socket = net::listen::_0_0_0_0(port)?;
  println!("> listening on 0.0.0.0:{port}");

  let candidates = Candidates::gather(&socket, port);
  candidates.show();

  let peer = net::listen::start(&socket)?;
  net::session::start(socket, peer)?;

  Ok(())
}

pub fn connect(target: &str) -> Result<()> {
  net::connect::start(target)
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

