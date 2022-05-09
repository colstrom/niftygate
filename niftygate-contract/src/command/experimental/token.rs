use super::super::{dump, CallReturn, SendReturn};
use crate::HexData;
use anyhow::Result;
use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  futures::StreamExt,
  Address, Bytes, U256,
};
use niftygate_bindings::benber86::nft_royalties_market::token::Contract;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoyaltyInfo {
  receiver: Address,
  amount: U256,
}

pub type RoyaltyInfoRaw = (Address, U256);

impl From<RoyaltyInfoRaw> for RoyaltyInfo {
  fn from((receiver, amount): RoyaltyInfoRaw) -> Self {
    Self { receiver, amount }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  #[structopt(long, value_name = "H160")]
  initial_royalties_receiver: Address,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(web3, self.initial_royalties_receiver)
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

  SupportsInterface,
  TokensOfOwner {
    #[structopt(long, value_name = "H160")]
    owner: Address,
  },
  RoyaltyInfo {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    sale_price: U256,
  },
}

impl CallCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::BalanceOf { owner }
        => contract.balance_of(owner).into(),
      Self::GetApproved { token_id }
        => contract.get_approved(token_id).into(),
      Self::IsApprovedForAll { owner, operator }
        => contract.is_approved_for_all(owner, operator).into(),
      Self::Name
        => contract.name().into(),
      Self::OwnerOf { token_id }
        => contract.owner_of(token_id).into(),
      Self::Symbol
        => contract.symbol().into(),
      Self::TokenByIndex { index }
        => contract.token_by_index(index).into(),
      Self::TokenOfOwnerByIndex { owner, index }
        => contract.token_of_owner_by_index(owner, index).into(),
      Self::TokenURI { token_id }
        => contract.token_uri(token_id).into(),
      Self::TotalSupply
        => contract.total_supply().into(),
      Self::SupportsInterface => {
        let interface_id: [u8; 4] = [0x2a, 0x55, 0x20, 0x5a];
        contract.supports_interface(Bytes(interface_id)).into()
      },
      Self::TokensOfOwner { owner } => contract.tokens_of_owner(owner).into(),
      Self::RoyaltyInfo {token_id, sale_price } => contract.royalty_info(token_id, sale_price).into(),
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
    about = "Mints a token.",
    long_about = "Mints <tokenId> and transfers it to <to>."
  )]
  Mint {
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long)]
    hash: String,
  },
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
  RoyaltiesReceiver,
  SetRoyaltiesReceiver {
    #[structopt(long, value_name = "H160")]
    new_royalties_receiver: Address,
  },
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::Approve { to, token_id }
        => contract.approve(to, token_id).into(),
      Self::Mint { recipient, hash }
        => contract.mint(recipient, hash).into(),
      Self::SafeTransferFrom { from, to, token_id, data }
        => match data {
          Some(data)
            => contract.safe_transfer_from_with_data(from, to, token_id, Bytes(data.0)).into(),
          None
            => contract.safe_transfer_from(from, to, token_id).into(),
        }
      Self::SetApprovalForAll { operator, approved }
        => contract.set_approval_for_all(operator, approved).into(),
      Self::TransferFrom { from, to, token_id }
        => contract.transfer_from(from, to, token_id).into(),
      Self::RoyaltiesReceiver => contract.royalties_receiver().into(),
      Self::SetRoyaltiesReceiver { new_royalties_receiver } => contract.set_royalties_receiver(new_royalties_receiver).into(),
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
    about = "Emitted when a token is transferred.",
    long_about = "Emitted when <tokenId> token is transferred from <from> to <to>."
  )]
  Transfer,
}

impl EventsCommand {
  pub async fn execute(self, web3: &DynWeb3, address: Address, stream: bool) -> Result<()> {
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
        Self::Transfer => {
          contract
            .events()
            .transfer()
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
        Self::Transfer => dump::query(contract.events().transfer().query().await?),
      }
    }

    Ok(())
  }
}
