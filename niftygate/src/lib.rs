use std::str::FromStr;

pub mod application;
pub mod command;
pub mod middleware;

pub use command::Command;
pub use niftygate_bindings::openzeppelin;

pub mod prelude {
  pub use ethcontract;
  pub use ethcontract::web3;
  pub use ethcontract::web3::ethabi;
  pub use surf;
  pub use tide;
}

#[deprecated(since = "0.7.1", note = "use anyhow::Error instead")]
pub type WrappedError = Box<dyn std::error::Error>;
#[allow(deprecated)]
#[deprecated(since = "0.7.1", note = "use anyhow::Result instead")]
pub type WrappedResult<T> = std::result::Result<T, WrappedError>;

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
#[derive(Debug)]
pub struct HexData(Vec<u8>);

impl FromStr for HexData {
  type Err = hex::FromHexError;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    hex::decode(s).map(Self)
  }
}
