use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
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
    let loopback = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), port);

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

  // target: 192.168.1.5
  // target: 50.50.50.50:5000
  // target: None
  // target: 127.0.0.1:6969
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

  // the announced link is a snapshot of the nat mapping.
  // waiting at the prompt sends nothing, so the mapping may expire and
  // come back on a different port, leaving the shared link stale.
  // returns (announced, current) only when the endpoint actually moved.
  pub fn recheck_public(&mut self, socket: &UdpSocket) -> Option<(SocketAddr, SocketAddr)> {
    let announced = self.public?;
    let current   = net::stun::discover(socket).ok()?;

    self.public = Some(current);

    (current != announced).then_some((announced, current))
  }
}

