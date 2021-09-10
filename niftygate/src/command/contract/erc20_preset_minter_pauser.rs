use super::{dump, CallReturn, SendReturn};
use crate::openzeppelin::contracts::token::erc20::presets::erc20_preset_minter_pauser::Contract;
use crate::WrappedResult;

use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  futures::StreamExt,
  Address, U256,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  #[structopt(long, value_name = "String")]
  name: String,
  #[structopt(long, value_name = "String")]
  symbol: String,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(web3, self.name, self.symbol)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum CallCommand {
  #[structopt(
    about = "Returns the allowance of one account on behalf of another.",
    long_about = "Returns the remaining number of tokens that <spender> will be allowed to spend on behalf of <owner> through {transferFrom}. This is zero by default."
  )]
  Allowance {
    #[structopt(long, value_name = "H160")]
    owner: Address,
    #[structopt(long, value_name = "H160")]
    spender: Address,
  },
  #[structopt(
    about = "Returns the number of tokens owned by an account.",
    long_about = "Returns the amount of tokens owned by <account>."
  )]
  BalanceOf {
    #[structopt(long, value_name = "H160")]
    account: Address,
  },
  #[structopt(about = "Returns the number of decimals used to get its user representation.")]
  Decimals,
  #[structopt(about = "Returns the name of the token.")]
  Name,
  #[structopt(about = "Returns true if the contract is paused, and false otherwise.")]
  Paused,
  #[structopt(
    about = "Returns the symbol of the token.",
    long_about = "Returns the symbol of the token, usually a shorter version of the name."
  )]
  Symbol,
  #[structopt(about = "Returns the amount of tokens in existence.")]
  TotalSupply,
}

impl CallCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::Allowance { owner, spender }
        => contract.allowance(owner, spender).into(),
      Self::BalanceOf { account }
        => contract.balance_of(account).into(),
      Self::Decimals
        => contract.decimals().into(),
      Self::Name
        => contract.name().into(),
      Self::Paused
        => contract.paused().into(),
      Self::Symbol
        => contract.symbol().into(),
      Self::TotalSupply
        => contract.total_supply().into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  #[structopt(
    about = "Sets the allowance over the caller's tokens.",
    long_about = "Sets <amount> as the allowance of <spender> over the caller's tokens."
  )]
  Approve {
    #[structopt(long, value_name = "H160")]
    spender: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(
    about = "Destroys tokens from the caller.",
    long_about = "Destroys <amount> tokens from the caller."
  )]
  Burn {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(
    about = "Destroys tokens, deducting from the caller's allowance.",
    long_about = "Destroys <amount> tokens from <account>, deducting from the caller's allowance."
  )]
  BurnFrom {
    account: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(
    about = "Atomically decreases the allowance granted by the caller.",
    long_about = "Atomically decreases the allowance granted to <spender> by the caller."
  )]
  DecreaseAllowance {
    #[structopt(long, value_name = "H160")]
    spender: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    subtracted_value: U256,
  },
  #[structopt(
    about = "Atomically increases the allowance granted by the caller.",
    long_about = "Atomically increases the allowance granted to <spender> by the caller."
  )]
  IncreaseAllowance {
    #[structopt(long, value_name = "H160")]
    spender: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    added_value: U256,
  },
  #[structopt(
    about = "Creates new tokens.",
    long_about = "Creates <amount> new tokens for <to>."
  )]
  Mint {
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(about = "Pauses all token transfers.")]
  Pause,
  #[structopt(
    about = "Moves tokens from the caller's account",
    long_about = "Moves <amount> tokens from the caller's account to <recipient>."
  )]
  Transfer {
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(
    about = "Moves tokens using the allowance mechanism.",
    long_about = "Moves <amount> tokens from <sender> to <recipient> using the allowance mechanism. <amount> is then deducted from the caller's allowance."
  )]
  TransferFrom {
    #[structopt(long, value_name = "H160")]
    sender: Address,
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(about = "Unpauses all token transfers.")]
  Unpause,
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::Approve { spender, amount }
        => contract.approve(spender, amount).into(),
      Self::Burn { amount }
        => contract.burn(amount).into(),
      Self::BurnFrom { account, amount }
        => contract.burn_from(account, amount).into(),
      Self::DecreaseAllowance { spender, subtracted_value }
        => contract.decrease_allowance(spender, subtracted_value).into(),
      Self::IncreaseAllowance { spender, added_value }
        => contract.increase_allowance(spender, added_value).into(),
      Self::Mint { to, amount }
        => contract.mint(to, amount).into(),
      Self::Pause
        => contract.pause().into(),
      Self::Transfer { recipient, amount }
        => contract.transfer(recipient, amount).into(),
      Self::TransferFrom { sender, recipient, amount }
        => contract.transfer_from(sender, recipient, amount).into(),
      Self::Unpause
        => contract.unpause().into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum EventsCommand {
  #[structopt(about = "Any events for this contract")]
  All,
  #[structopt(
    about = "Emitted when the allowance of an account is set for another.",
    long_about = "Emitted when the allowance of a <spender> for an <owner> is set by a call to {approve}. <value> is the new allowance."
  )]
  Approval,
  #[structopt(
    about = "Emitted when the pause is triggered.",
    long_about = "Emitted when the pause is triggered by <account>."
  )]
  Paused,
  #[structopt(
    about = "Emitted when an admin role is replaced.",
    long_about = "Emitted when <newAdminRole> is set as <role>'s admin role, replacing <previousAdminRole>"
  )]
  RoleAdminChanged,
  #[structopt(
    about = "Emitted when a role is granted.",
    long_about = "Emitted when <account> is granted <role>."
  )]
  RoleGranted,
  #[structopt(
    about = "Emitted when a role is revoked.",
    long_about = "Emitted when <account> is revoked <role>."
  )]
  RoleRevoked,
  #[structopt(
    about = "Emitted when tokens are moved from one account to another.",
    long_about = "Emitted when <value> tokens are moved from one account (<from>) to another (<to>)."
  )]
  Transfer,
  #[structopt(
    about = "Emitted when the pause is lifted.",
    long_about = "Emitted when the pause is lifted by <account>."
  )]
  Unpaused,
}

impl EventsCommand {
  pub async fn execute(self, web3: &DynWeb3, address: Address, stream: bool) -> WrappedResult<()> {
    let contract = Contract::at(web3, address);

    if stream {
      match self {
        Self::All => contract.all_events().stream().for_each(dump::stream).await,
        Self::Approval => {
          contract
            .events()
            .approval()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::Paused => {
          contract
            .events()
            .paused()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::RoleAdminChanged => {
          contract
            .events()
            .role_admin_changed()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::RoleGranted => {
          contract
            .events()
            .role_granted()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::RoleRevoked => {
          contract
            .events()
            .role_revoked()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::Transfer => {
          contract
            .events()
            .transfer()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::Unpaused => {
          contract
            .events()
            .unpaused()
            .stream()
            .for_each(dump::stream)
            .await
        }
      }
    } else {
      match self {
        Self::All => dump::query(contract.all_events().query().await?),
        Self::Approval => dump::query(contract.events().approval().query().await?),
        Self::Paused => dump::query(contract.events().paused().query().await?),
        Self::RoleAdminChanged => {
          dump::query(contract.events().role_admin_changed().query().await?)
        }
        Self::RoleGranted => dump::query(contract.events().role_granted().query().await?),
        Self::RoleRevoked => dump::query(contract.events().role_revoked().query().await?),
        Self::Transfer => dump::query(contract.events().transfer().query().await?),
        Self::Unpaused => dump::query(contract.events().unpaused().query().await?),
      }
    }

    Ok(())
  }
}
