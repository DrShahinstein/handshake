use crate::{Result, HshError, cli};
use crate::net::{self, Frame, FrameType, candidate::Candidates};
use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;

pub fn start(target: &str) -> Result<()> {
  let candidates = resolve_target(target)?;
  let syn_packet = Frame::syn();
  let socket     = UdpSocket::bind("0.0.0.0:0")?;

  socket.set_read_timeout(Some(Duration::from_secs(3)))?;

  for addr in candidates {
    net::send_frame_to(&socket, &syn_packet, addr)?;
  }

  let (f, src) = net::recv_frame(&socket)?;

  if f.frame_type != FrameType::Ack {
    return Err(HshError::ExpectedAck);
  }

  net::session::start(socket, src)?;

  Ok(())
}

pub fn resolve_target(target: &str) -> Result<Vec<SocketAddr>> {
  let t = target.trim();

  if t.starts_with(net::LINK_PREFIX) {
    Candidates::from_base64(t)
  }
  
  else {
    let normalized       = normalize_target(t);
    let addr: SocketAddr = normalized.parse().map_err(|_| HshError::InvalidLink)?;
    Ok(vec![addr])
  }
}

pub fn normalize_target(target: &str) -> String {
  let default_port = cli::DEFAULT_PORT;

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
