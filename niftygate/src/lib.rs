pub mod application;
pub mod command;
pub mod middleware;

pub use niftygate_bindings::openzeppelin;

pub mod prelude {
  pub use ethcontract;
  pub use ethcontract::web3;
  pub use ethcontract::web3::ethabi;
  pub use surf;
  pub use tide;
}

pub type WrappedError = Box<dyn std::error::Error>;
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
