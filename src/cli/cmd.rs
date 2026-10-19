use crate::{Result, cli};

pub fn listen(port: u16) -> Result<()> {
  println!("entered listen with port: {}", port);
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

