use crate::Result;

pub fn listen(port: u16) -> Result<()> {
  println!("entered listen with port: {}", port);
  Ok(())
}

pub fn connect(target: &str) -> Result<()> {
  println!("entered connect with target: {}", target);
  Ok(())
}

pub fn help() -> Result<()> {
  println!("help message");
  Ok(())
}

pub fn version() -> Result<()> {
  println!("version 0.1.0");
  Ok(())
}

