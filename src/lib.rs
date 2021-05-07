pub mod application;
pub mod command;
pub mod middleware;
pub mod openzeppelin;

pub mod prelude {
  pub use ethcontract;
  pub use ethcontract::web3;
  pub use ethcontract::web3::ethabi;
  pub use surf;
  pub use tide;
}

pub type WrappedError = Box<dyn std::error::Error>;
pub type WrappedResult<T> = std::result::Result<T, WrappedError>;
