pub mod cli;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HshError {
  #[error("{0}")]
  IoErr(#[from] std::io::Error),

  #[error("No such command.")]
  CliNoCmd,

  #[error("Expected argument.")]
  CliNoArg,
}

pub type Result<T> = std::result::Result<T, HshError>;

