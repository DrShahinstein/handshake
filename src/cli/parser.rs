use crate::{Result, HshError, cli};
use std::env;

pub fn run() -> Result<cli::Command> {
  let mut args = env::args().skip(1);

  let command = match args.next() {
    Some(s) => s,
    None    => return Ok(cli::Command::Help),
  };

  match command.as_str() {
    "listen" => {
      let port_str  = args.next().unwrap_or_else(|| cli::DEFAULT_PORT.to_string());
      let port: u16 = port_str.parse().unwrap_or(cli::DEFAULT_PORT);
      Ok(cli::Command::Listen {port})
    }

    "connect" => {
      let target = args.next().ok_or(HshError::CliNoArg)?;
      Ok(cli::Command::Connect {target})
    }

    "help"    => Ok(cli::Command::Help),
    "version" => Ok(cli::Command::Version),
    __other__ => Err(HshError::CliNoCmd)
  }
}

