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
Usage: handshake punch [OPTIONS] [PEER]

Arguments:
  [PEER]  base64-link or IP:PORT

Options:
  -p, --port <PORT>  Local UDP port to bind [default: 6969]
  -h, --help         Print help information
  -V, --version      Print version information
"
;
