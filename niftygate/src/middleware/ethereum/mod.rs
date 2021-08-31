pub mod account;
pub mod balance;
pub mod signature;

pub mod erc1155;
pub mod erc20;
pub mod erc721;
pub mod erc777;

pub mod prelude {
  pub use super::account::prelude::*;
  pub use super::balance::prelude::*;
  pub use super::erc1155::prelude::*;
  pub use super::erc20::prelude::*;
  pub use super::erc721::prelude::*;
  pub use super::erc777::prelude::*;
  pub use super::signature::prelude::*;
}

pub use account::ProvidesAccountVerification;
pub use balance::{BalanceRequirement, BalanceScale, ProvidesBalance, RequiresBalance};
pub use erc1155::ProvidesERC1155Balance;
pub use erc20::ProvidesERC20Balance;
pub use erc721::ProvidesERC721Balance;
pub use erc777::ProvidesERC777Balance;
pub use signature::ProvidesSignature;
