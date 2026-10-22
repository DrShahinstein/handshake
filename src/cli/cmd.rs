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

  let mut buf = [0u8; 256];

  let peer = loop {
    let (bytes, src) = socket.recv_from(&mut buf)?;
    if bytes < 7 {
      continue;
    }

    let payload = &buf[..bytes];

    if &payload[0..7] != b"HSH_SYN" {
      socket.send_to(b"i don't speak this language", src)?;
      continue;
    }

    socket.send_to(b"HSH_ACK", src)?;

    break src;
  };

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

