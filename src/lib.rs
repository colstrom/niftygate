pub mod application;
pub mod command;
pub mod middleware;

pub mod prelude {
  pub use tide;
  pub use web3;
}

pub type WrappedError = Box<dyn std::error::Error>;
pub type WrappedResult<T> = std::result::Result<T, WrappedError>;
