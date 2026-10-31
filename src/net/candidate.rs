use std::net::{SocketAddr, UdpSocket};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use crate::{HshError, Result, net};

#[derive(Debug, Clone, PartialEq)]
pub struct Candidates {
  pub loopback: SocketAddr,
  pub lan: Option<SocketAddr>,
  pub public: Option<SocketAddr>,
}

impl Candidates {
  pub fn gather(socket: &UdpSocket, port: u16) -> Self {
    let loopback: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();

    let lan = net::get_lan_ip()
               .and_then(|ip| format!("{ip}:{port}")
               .parse()
               .ok());

    let public = if lan.is_some() {
      net::stun::discover(socket).ok()
    } else {
      None
    };

    Self {
      loopback,
      lan,
      public,
    }
  }

  pub fn addrs(&self) -> Vec<SocketAddr> {
    let mut list = Vec::new();

    if let Some(lan) = self.lan {
      list.push(lan);
    }
    if let Some(public) = self.public {
      list.push(public);
    }

    list
  }

  pub fn to_base64(&self) -> String {
    let addrs_str = self
      .addrs()
      .iter()
      .map(|addr| addr.to_string())
      .collect::<Vec<_>>()
      .join(",");

    let encoded = URL_SAFE_NO_PAD.encode(addrs_str.as_bytes());

    format!("{}{}", net::LINK_PREFIX, encoded)
  }

  pub fn from_base64(link: &str) -> Result<Vec<SocketAddr>> {
    let clean_link = link
      .trim()
      .strip_prefix(net::LINK_PREFIX)
      .ok_or(HshError::InvalidLink)?;

    let decoded_bytes = URL_SAFE_NO_PAD
      .decode(clean_link.as_bytes())
      .map_err(|_| HshError::InvalidLink)?;

    let decoded_str = String::from_utf8(decoded_bytes).map_err(|_| HshError::InvalidLink)?;

    let addrs: Vec<SocketAddr> = decoded_str
      .split(',')
      .filter_map(|addr| addr.parse::<SocketAddr>().ok())
      .filter(|addr| !addr.ip().is_loopback())
      .collect();

    if addrs.is_empty() {
      return Err(HshError::InvalidLink);
    }

    Ok(addrs)
  }

  pub fn resolve_target(target: &str) -> Result<Vec<SocketAddr>> {
    let target = target.trim();

    if target.starts_with(net::LINK_PREFIX) {
      Self::from_base64(target)
    } else {
      target
        .parse::<SocketAddr>()
        .map(|addr| vec![addr])
        .map_err(|_| HshError::InvalidLink)
    }
  }

  pub fn show(&self) {
    println!("  * loopback: {}", self.loopback);

    if let Some(lan) = self.lan {
      println!("  * lan: {lan}");
    } else {
      println!("  ! lan: offline");
    }

    if let Some(public) = self.public {
      println!("> discovered public nat endpoint: \x1b[1;32m{public}\x1b[0m");
    } else {
      println!("> nat discovery failed or skipped.");
    }

    let raw_link    = self.to_base64();
    let styled_link = format!("\x1b[4;36m{raw_link}\x1b[0m");

    println!("> share this link with your peer:\n{styled_link}\n");
  }
}

