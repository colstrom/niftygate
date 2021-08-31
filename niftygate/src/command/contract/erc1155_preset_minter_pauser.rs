use super::{dump, CallReturn, SendReturn};
use crate::openzeppelin::contracts::token::erc1155::presets::erc1155_preset_minter_pauser::Contract;
use crate::{command::HexData, WrappedResult};
use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  futures::StreamExt,
  Address, Bytes, U256,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  #[structopt(long, value_name = "String")]
  uri: String,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(web3, self.uri)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum CallCommand {
  #[structopt(
    about = "Returns the amount of tokens of a given type owned by an account.",
    long_about = "Returns the amount of tokens of token type <id> owned by <account>."
  )]
  BalanceOf {
    #[structopt(long, value_name = "H160")]
    account: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    id: U256,
  },
  #[structopt(about = "Batched version of {balanceOf}.")]
  BalanceOfBatch {
    #[structopt(long, value_name = "H160", use_delimiter = true)]
    accounts: Vec<Address>,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    ids: Vec<U256>,
  },
  #[structopt(
    about = "Returns true if an account is approved to transfer another's tokens.",
    long_about = "Returns true if <operator> is approved to transfer <account>'s tokens."
  )]
  IsApprovedForAll {
    #[structopt(long, value_name = "H160")]
    account: Address,
    #[structopt(long, value_name = "H160")]
    operator: Address,
  },
  #[structopt(about = "Returns true if the contract is paused, and false otherwise.")]
  Paused,
  #[structopt(
    about = "Returns the URI for a given token type.",
    long_about = "Returns the URI for token type <id>. If the {id} substring is present in the URI, it must be replaced by clients with the actual token type ID."
  )]
  URI {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    p0: U256,
  },
}

impl CallCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::BalanceOf { account, id }
        => contract.balance_of(account, id).into(),
      Self::BalanceOfBatch { accounts, ids }
        => contract.balance_of_batch(accounts, ids).into(),
      Self::IsApprovedForAll { account, operator }
        => contract.is_approved_for_all(account, operator).into(),
      Self::Paused
        => contract.paused().into(),
      Self::URI { p0 }
        => contract.uri(p0).into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  #[structopt(
    about = "Destroys tokens.",
    long_about = "Destroys <value> tokens for <account> of token type <id>."
  )]
  Burn {
    #[structopt(long, value_name = "H160")]
    account: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    value: U256,
  },
  #[structopt(about = "Batched version of {burn}.")]
  BurnBatch {
    #[structopt(long, value_name = "H160")]
    account: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    ids: Vec<U256>,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    values: Vec<U256>,
  },
  #[structopt(
    about = "Creates new tokens.",
    long_about = "Creates <amount> new tokens for <to>, of token type <id>."
  )]
  Mint {
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
  },
  #[structopt(about = "Batched version of {mint}.")]
  MintBatch {
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    ids: Vec<U256>,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    amounts: Vec<U256>,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
  },
  #[structopt(about = "Pauses all token transfers.")]
  Pause,
  #[structopt(about = "Batched version of {safeTransferFrom}.")]
  SafeBatchTransferFrom {
    #[structopt(long, value_name = "H160")]
    from: Address,
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    ids: Vec<U256>,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str), use_delimiter = true)]
    amounts: Vec<U256>,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
  },
  #[structopt(
    about = "Transfers tokens from one account to another.",
    long_about = "Transfers <amount> tokens of token type <id> from <from> to <to>."
  )]
  SafeTransferFrom {
    #[structopt(long, value_name = "H160")]
    from: Address,
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
  },
  #[structopt(
    about = "Grants or revokes permission to transfer the caller's tokens.",
    long_about = "Grants or revokes permission to <operator> to transfer the caller's tokens, according to <approved>."
  )]
  SetApprovalForAll {
    #[structopt(long, value_name = "H160")]
    operator: Address,
    #[structopt(long)]
    approved: bool,
  },
  #[structopt(about = "Unpauses all token transfers.")]
  Unpause,
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(&web3, address);
    match self {
      Self::Burn { account, id, value }
        => contract.burn(account, id, value).into(),
      Self::BurnBatch { account, ids, values }
        => contract.burn_batch(account, ids, values).into(),
      Self::Mint { to, id, amount, data }
        => contract.mint(to, id, amount, Bytes(data.0)).into(),
      Self::MintBatch { to, ids, amounts, data }
        => contract.mint_batch(to, ids, amounts, Bytes(data.0)).into(),
      Self::Pause
        => contract.pause().into(),
      Self::SafeBatchTransferFrom { from, to, ids, amounts, data }
        => contract.safe_batch_transfer_from(from, to, ids, amounts, Bytes(data.0)).into(),
      Self::SafeTransferFrom { from, to, id, amount, data }
        => contract.safe_transfer_from(from, to, id, amount, Bytes(data.0)).into(),
      Self::SetApprovalForAll { operator, approved }
        => contract.set_approval_for_all(operator, approved).into(),
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
    about = "Emitted when permission to transfer tokens is granted or revoked.",
    long_about = "Emitted when <account> grants or revokes permission to <operator> to transfer their tokens, according to <approved>."
  )]
  ApprovalForAll,
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
    about = "Equivalent to multiple {TransferSingle} events.",
    long_about = "Equivalent to multiple {TransferSingle} events, where <operator>, <from> and <to> are the same for all transfers."
  )]
  TransferBatch,
  #[structopt(
    about = "Emitted when tokens are transferred.",
    long_about = "Emitted when <value> tokens of token type <id> are transferred from <from> to <to> by <operator>."
  )]
  TransferSingle,
  #[structopt(
    about = "Emitted when the pause is lifted.",
    long_about = "Emitted when the pause is lifted by <account>."
  )]
  Unpaused,
  #[structopt(
    about = "Emitted when URI changes for a token.",
    long_about = "Emitted when the URI for token type <id> changes to <value>, if it is a non-programmatic URI."
  )]
  URI,
}

impl EventsCommand {
  pub async fn execute(self, web3: &DynWeb3, address: Address, stream: bool) -> WrappedResult<()> {
    let contract = Contract::at(web3, address);

    if stream {
      match self {
        Self::All => contract.all_events().stream().for_each(dump::stream).await,
        Self::ApprovalForAll => {
          contract
            .events()
            .approval_for_all()
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
        Self::TransferBatch => {
          contract
            .events()
            .transfer_batch()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::TransferSingle => {
          contract
            .events()
            .transfer_single()
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
        Self::URI => {
          contract
            .events()
            .uri()
            .stream()
            .for_each(dump::stream)
            .await
        }
      }
    } else {
      match self {
        Self::All => dump::query(contract.all_events().query().await?),
        Self::ApprovalForAll => dump::query(contract.events().approval_for_all().query().await?),
        Self::Paused => dump::query(contract.events().paused().query().await?),
        Self::RoleAdminChanged => {
          dump::query(contract.events().role_admin_changed().query().await?)
        }
        Self::RoleGranted => dump::query(contract.events().role_granted().query().await?),
        Self::RoleRevoked => dump::query(contract.events().role_revoked().query().await?),
        Self::TransferBatch => dump::query(contract.events().transfer_batch().query().await?),
        Self::TransferSingle => dump::query(contract.events().transfer_single().query().await?),
        Self::Unpaused => dump::query(contract.events().unpaused().query().await?),
        Self::URI => dump::query(contract.events().uri().query().await?),
      }
    }

    Ok(())
  }
}
