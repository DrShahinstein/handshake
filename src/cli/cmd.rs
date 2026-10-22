use crate::{Result, cli, net};

pub fn listen(port: u16) -> Result<()> {
  let _socket = net::listen::_0_0_0_0(port)?;
  let lan_ip = net::listen::get_lan_ip();

  println!("> listening on 0.0.0.0:{port}");
  println!("  * loopback: 127.0.0.1:{port}");

  let Some(ip) = lan_ip else {
    println!("  ! lan: offline");
    println!("> skipped nat discovery");
    return Ok(());
  };

  println!("  * lan: {ip}:{port}");
  println!("> discovery here...");

  Ok(())
}

pub fn connect(target: &str) -> Result<()> {
  println!("entered connect with target: {}", target);
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

