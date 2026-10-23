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
      let target = resolve_target(&target);
      Ok(cli::Command::Connect {target})
    }

    "help"    => Ok(cli::Command::Help),
    "version" => Ok(cli::Command::Version),
    __other__ => Err(HshError::CliNoCmd)
  }
}

fn resolve_target(target: &str) -> String {
  let default_port = crate::cli::DEFAULT_PORT;

  if let Ok(port) = target.parse::<u16>() {
    return format!("127.0.0.1:{port}");
  }
  if target == "localhost" || target == "loopback" {
    return format!("127.0.0.1:{default_port}");
  }
  if let Some(port) = target.strip_prefix("localhost:").or_else(|| target.strip_prefix("loopback:")) {
    return format!("127.0.0.1:{port}");
  }
  if !target.contains(':') {
    return format!("{target}:{default_port}");
  }

  target.to_string()
}

