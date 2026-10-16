use handshake::cli;

fn main() {
  cli::launch().unwrap_or_else(|e| eprintln!("{e}"));
}

