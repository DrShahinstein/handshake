use crate::Result;

pub mod cmd;
pub mod parser;

pub const DEFAULT_PORT: u16 = 6969;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
  Punch {port: u16, target: Option<String>},
  Help,
  Version,
}

pub fn launch() -> Result<()> {
  match parser::run()? {
    Command::Punch {port,target} => cmd::punch(port, target.as_deref()),
    Command::Help                => cmd::help(),
    Command::Version             => cmd::version(),
  }
}

pub const HELP_MSG: &str = "\
Usage: handshake <command>

Commands:
 - punch [IP:PORT / base64-link]
 - help
 - version

Options:
  -p, --port <PORT>  Local UDP port to bind [default: 6969]
";

