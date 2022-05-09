use std::str::FromStr;

pub mod command;

pub use command::Command;

#[derive(Debug)]
pub struct HexData(Vec<u8>);

impl FromStr for HexData {
  type Err = hex::FromHexError;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    hex::decode(s).map(Self)
  }
}

mod util {
  use ethcontract::{
    dyns::DynWeb3,
    transport::DynTransport,
    web3::{error::Result, transports::WebSocket},
    Web3,
  };
  use tide::http::Url;

  pub async fn web3_from_url(url: Url) -> Result<DynWeb3> {
    Ok(Web3::new(DynTransport::new(
      WebSocket::new(url.as_str()).await?,
    )))
  }
}
