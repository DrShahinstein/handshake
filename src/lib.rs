pub mod cli;
pub mod net;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HshError {
  #[error("IO Error: {0}")]
  IoErr(#[from] std::io::Error),

  #[error("No such command.")]
  CliNoCmd,

  #[error("Expected argument.")]
  CliNoArg,

  #[error("Invalid handshake link.")]
  InvalidLink,
}

pub type Result<T> = std::result::Result<T, HshError>;

