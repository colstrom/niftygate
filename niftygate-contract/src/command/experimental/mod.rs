use super::{CallReturn, SendReturn};
use anyhow::Result;
use ethcontract::{dyns::DynWeb3, Account, Address};
use structopt::StructOpt;

pub(super) mod crowdsale;
pub(super) mod marketplace;
pub(super) mod royalties_payment;
pub(super) mod token;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum DeployVariant {
  Crowdsale(crowdsale::DeployCommand),
  Marketplace(marketplace::DeployCommand),
  RoyaltiesPayment(royalties_payment::DeployCommand),
  Token(token::DeployCommand),
}

impl DeployVariant {
  pub async fn execute(self, web3: &DynWeb3, account: Account) -> Result<Address> {
    let address = match self {
      Self::Crowdsale(variant) => variant.build(web3)?.from(account).deploy().await?.address(),
      Self::Marketplace(variant) => variant.build(web3).from(account).deploy().await?.address(),
      Self::RoyaltiesPayment(variant) => {
        variant.build(web3).from(account).deploy().await?.address()
      }
      Self::Token(variant) => variant.build(web3).from(account).deploy().await?.address(),
    };
    Ok(address)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum CallVariant {
  Crowdsale(crowdsale::CallCommand),
  Token(token::CallCommand),
}

impl CallVariant {
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    match self {
      Self::Crowdsale(variant) => variant.build(web3, address),
      Self::Token(variant) => variant.build(web3, address),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum SendVariant {
  Crowdsale(crowdsale::SendCommand),
  Marketplace(marketplace::SendCommand),
  RoyaltiesPayment(royalties_payment::SendCommand),
  Token(token::SendCommand),
}

impl SendVariant {
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    match self {
      Self::Crowdsale(variant) => variant.build(web3, address),
      Self::Marketplace(variant) => variant.build(web3, address),
      Self::RoyaltiesPayment(variant) => variant.build(web3, address),
      Self::Token(variant) => variant.build(web3, address),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum EventsVariant {
  Crowdsale(crowdsale::EventsCommand),
  Marketplace(marketplace::EventsCommand),
  Token(token::EventsCommand),
}

impl EventsVariant {
  pub async fn execute(self, web3: &DynWeb3, address: Address, stream: bool) -> Result<()> {
    match self {
      Self::Crowdsale(variant) => variant.execute(web3, address, stream).await?,
      Self::Marketplace(variant) => variant.execute(web3, address, stream).await?,
      Self::Token(variant) => variant.execute(web3, address, stream).await?,
    }

    Ok(())
  }
}
