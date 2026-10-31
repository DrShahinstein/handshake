use std::process::ExitCode;
use handshake::cli;

fn main() -> ExitCode {
  match cli::launch() {
    Ok(()) => ExitCode::SUCCESS,
    Err(err) => {
      eprintln!("{err}");
      ExitCode::FAILURE
    }
  }
}

