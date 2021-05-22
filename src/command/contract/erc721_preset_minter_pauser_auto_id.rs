use super::{dump, CallReturn, SendReturn};
use crate::openzeppelin::contracts::token::erc721::presets::erc721_preset_minter_pauser_auto_id::Contract;
use crate::{command::HexData, WrappedResult};
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
  #[structopt(long = "baseTokenURI", value_name = "String")]
  base_token_uri: String,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(web3, self.name, self.symbol, self.base_token_uri)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum CallCommand {
  #[structopt(
    about = "Returns the number of tokens in an account.",
    long_about = "Returns the number of tokens in <owner>'s account."
  )]
  BalanceOf {
    #[structopt(long, value_name = "H160")]
    owner: Address,
  },
  #[structopt(
    about = "Returns the account approved for a token.",
    long_about = "Returns the account approved for <tokenId> token."
  )]
  GetApproved {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  #[structopt(
    about = "Returns if an account is allowed to manage another's assets.",
    long_about = "Returns if the <operator> is allowed to manage all of the assets of <owner>."
  )]
  IsApprovedForAll {
    #[structopt(long, value_name = "H160")]
    owner: Address,
    #[structopt(long, value_name = "H160")]
    operator: Address,
  },
  #[structopt(about = "Returns the token collection name.")]
  Name,
  #[structopt(
    about = "Returns the owner of a token.",
    long_about = "Returns the owner of the <tokenId> token."
  )]
  OwnerOf {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  #[structopt(about = "Returns true if the contract is paused, and false otherwise.")]
  Paused,
  #[structopt(about = "Returns the token collection symbol.")]
  Symbol,
  #[structopt(
    about = "Returns a token ID from the contract's tokens.",
    long_about = "Returns a token ID at a given `index` of all the tokens stored by the contract."
  )]
  TokenByIndex {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    index: U256,
  },
  #[structopt(
    about = "Returns a token ID from an account's tokens.",
    long_about = "Returns a token ID owned by <owner> at a given <index> of its token list."
  )]
  TokenOfOwnerByIndex {
    #[structopt(long, value_name = "H160")]
    owner: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    index: U256,
  },
  #[structopt(
    about = "Returns the URI for a token",
    long_about = "Returns the Uniform Resource Identifier (URI) for <tokenId> token."
  )]
  TokenURI {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  #[structopt(about = "Returns the total amount of tokens stored by the contract.")]
  TotalSupply,
}

impl CallCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::BalanceOf { owner }
        => contract.balance_of(owner).view().into(),
      Self::GetApproved { token_id }
        => contract.get_approved(token_id).view().into(),
      Self::IsApprovedForAll { owner, operator }
        => contract.is_approved_for_all(owner, operator).view().into(),
      Self::Name
        => contract.name().view().into(),
      Self::OwnerOf { token_id }
        => contract.owner_of(token_id).view().into(),
      Self::Paused
        => contract.paused().view().into(),
      Self::Symbol
        => contract.symbol().view().into(),
      Self::TokenByIndex { index }
        => contract.token_by_index(index).view().into(),
      Self::TokenOfOwnerByIndex { owner, index }
        => contract.token_of_owner_by_index(owner, index).view().into(),
      Self::TokenURI { token_id }
        => contract.token_uri(token_id).view().into(),
      Self::TotalSupply
        => contract.total_supply().view().into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  #[structopt(
    about = "Gives permission to transfer a token to another account.",
    long_about = "Gives permission to `to` to transfer `tokenId` token to another account. The approval is cleared when the token is transferred."
  )]
  Approve {
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  #[structopt(
    about = "Destroys a token.",
    long_about = "Destroys <tokenId>. The approval is cleared when the token is burned."
  )]
  Burn {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  #[structopt(
    about = "Mints a token.",
    long_about = "Mints <tokenId> and transfers it to <to>."
  )]
  Mint {
    #[structopt(long, value_name = "H160")]
    to: Address,
  },
  #[structopt(about = "Pauses all token transfers.")]
  Pause,
  #[structopt(
    about = "Safely transfers a token.",
    long_about = "Safely transfers <tokenId> token from <from> to <to>, checking first that contract recipients are aware of the ERC721 protocol to prevent tokens from being forever locked."
  )]
  SafeTransferFrom {
    #[structopt(long, value_name = "H160")]
    from: Address,
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
    #[structopt(long, value_name = "HexString")]
    data: Option<HexData>,
  },
  #[structopt(
    about = "Approve or remove an operator for the caller.",
    long_about = "Approve or remove <operator> as an operator for the caller. Operators can call {transferFrom} or {safeTransferFrom} for any token owned by the caller."
  )]
  SetApprovalForAll {
    #[structopt(long, value_name = "H160")]
    operator: Address,
    #[structopt(long)]
    approved: bool,
  },
  #[structopt(
    about = "Gives permission to transfer a token to another account.",
    long_about = "Gives permission to `to` to transfer `tokenId` token to another account. The approval is cleared when the token is transferred."
  )]
  TransferFrom {
    #[structopt(long, value_name = "H160")]
    from: Address,
    #[structopt(long, value_name = "H160")]
    to: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  #[structopt(about = "Unpauses all token transfers.")]
  Unpause,
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(&web3, address);
    match self {
      Self::Approve { to, token_id }
        => contract.approve(to, token_id).into(),
      Self::Burn { token_id }
        => contract.burn(token_id).into(),
      Self::Mint { to }
        => contract.mint(to).into(),
      Self::Pause
        => contract.pause().into(),
      Self::SafeTransferFrom { from, to, token_id, data }
        => match data {
          Some(data)
            => contract.safe_transfer_from_with_data(from, to, token_id, data.0).into(),
          None
            => contract.safe_transfer_from(from, to, token_id).into(),
        }
      Self::SetApprovalForAll { operator, approved }
        => contract.set_approval_for_all(operator, approved).into(),
      Self::TransferFrom { from, to, token_id }
        => contract.transfer_from(from, to, token_id).into(),
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
    about = "Emitted when an account allows another to manage some of its assets.",
    long_about = "Emitted when <owner> enables <approved> to manage the <tokenId> token."
  )]
  Approval,
  #[structopt(
    about = "Emitted when an account (dis)allows another to manage all of its assets.",
    long_about = "Emitted when <owner> enables or disables (<approved>) <operator> to manage all of its assets."
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
    about = "Emitted when a token is transferred.",
    long_about = "Emitted when <tokenId> token is transferred from <from> to <to>."
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
        Self::ApprovalForAll => dump::query(contract.events().approval_for_all().query().await?),
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
