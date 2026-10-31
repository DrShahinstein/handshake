pub mod cli;
pub mod net;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HshError {
  #[error("IO Error: {0}")]
  IoErr(#[from] std::io::Error),

  #[error("No such command: {0}")]
  CliNoCmd(String),

  #[error("Expected argument for {0}.")]
  CliNoArg(String),

  #[error("Unknown option: {0}")]
  CliUnknownOpt(String),

  #[error("Too many arguments.")]
  CliTooManyArgs,

  #[error("Invalid port: {0}")]
  CliInvalidPort(String),

  #[error("Invalid handshake link.")]
  InvalidLink,

  #[error("Handshake failed: expected ACK frame.")]
  ExpectedAck,
}

pub type Result<T> = std::result::Result<T, HshError>;

