use crate::{HshError, Result, cli};
use std::env;

pub fn run() -> Result<cli::Command> {
  parse(env::args().skip(1))
}

pub fn parse<I>(args: I) -> Result<cli::Command>
where I: IntoIterator<Item = String>
{
  let mut args = args.into_iter();

  let command = match args.next() {
    Some(command) => command,
    None          => return Ok(cli::Command::Help),
  };

  match command.as_str() {
    "punch"   => parse_punch(args),
    "help"    => Ok(cli::Command::Help),
    "version" => Ok(cli::Command::Version),
    __other__ => Err(HshError::CliNoCmd(__other__.to_string())),
  }
}

fn parse_punch<I>(mut args: I) -> Result<cli::Command>
where I: Iterator<Item = String>
{
  let mut port   = cli::DEFAULT_PORT;
  let mut target = None;

  while let Some(arg) = args.next() {
    if arg == "-p" || arg == "--port" {
      let value = args.next().ok_or_else(|| HshError::CliNoArg(arg.clone()))?;
      port      = value.parse().map_err(|_| HshError::CliInvalidPort(value))?;
      continue;
    }

    if arg.starts_with('-') {
      return Err(HshError::CliUnknownOpt(arg));
    }

    if target.is_some() {
      return Err(HshError::CliTooManyArgs);
    }

    target = Some(arg);
  }

  Ok(cli::Command::Punch {port, target})
}

