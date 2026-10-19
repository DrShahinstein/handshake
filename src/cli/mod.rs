use crate::Result;

pub mod cmd;
pub mod parser;

pub const DEFAULT_PORT: u16 = 6969;

pub enum Command {
  Listen  {port: u16},
  Connect {target: String},
  Help,
  Version,
}

pub fn launch() -> Result<()> {
  match parser::run()? {
    Command::Listen  {port}   => cmd::listen(port),
    Command::Connect {target} => cmd::connect(&target),
    Command::Help             => cmd::help(),
    Command::Version          => cmd::version(),
  }
}

pub const HELP_MSG: &str = "\
Usage: handshake <command> <argument>

Commands:
- listen <port:6969>
- connect <socket>
"
;
