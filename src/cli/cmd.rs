use crate::{Result, cli, net};

pub fn listen(port: u16) -> Result<()> {
  let socket = net::listen::_0_0_0_0(port)?;
  let lan_ip = net::listen::get_lan_ip();

  println!("> listening on 0.0.0.0:{port}");
  println!("  * loopback: 127.0.0.1:{port}");

  if let Some(ip) = lan_ip {
    println!("  * lan: {ip}:{port}");
    println!("> discovery here...");
  } else {
    println!("  ! lan: offline");
    println!("> skipped nat discovery");
  } 

  let peer = net::listen::start(&socket)?;

  net::session::start(socket, peer)?;

  Ok(())
}

pub fn connect(target: &str) -> Result<()> {
  net::connect::start(target)?;
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

