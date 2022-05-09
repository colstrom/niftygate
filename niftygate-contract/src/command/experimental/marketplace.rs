use super::super::{dump, SendReturn};
use anyhow::Result;
use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  futures::StreamExt,
  Address, U256,
};
use niftygate_bindings::benber86::nft_royalties_market::marketplace::Contract;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  #[structopt(long, value_name = "H160")]
  token_contract_address: Address,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(web3, self.token_contract_address)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  MakeSellOffer {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    min_price: U256,
  },
  WithdrawSellOffer {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  MakeBuyOffer {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    value: U256,
  },
  WithdrawBuyOffer {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  AcceptBuyOffer {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
  },
  Purchase {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    token_id: U256,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    value: U256,
  },
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::MakeSellOffer { token_id, min_price } => contract.make_sell_offer(token_id, min_price).into(),
      Self::WithdrawSellOffer { token_id } => contract.withdraw_sell_offer(token_id).into(),
      Self::MakeBuyOffer { token_id, value } => contract.make_buy_offer(token_id).value(value).into(),
      Self::WithdrawBuyOffer { token_id } => contract.withdraw_buy_offer(token_id).into(),
      Self::AcceptBuyOffer { token_id } => contract.accept_buy_offer(token_id).into(),
      Self::Purchase { token_id, value } => contract.make_buy_offer(token_id).value(value).into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum EventsCommand {
  All,
  NewSellOffer,
  SellOfferWithdrawn,
  NewBuyOffer,
  BuyOfferWithdrawn,
  Sale,
  RoyaltiesPaid,
}

impl EventsCommand {
  pub async fn execute(self, web3: &DynWeb3, address: Address, stream: bool) -> Result<()> {
    let contract = Contract::at(web3, address);

    if stream {
      match self {
        Self::All => contract.all_events().stream().for_each(dump::stream).await,
        Self::NewSellOffer => {
          contract
            .events()
            .new_sell_offer()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::SellOfferWithdrawn => {
          contract
            .events()
            .sell_offer_withdrawn()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::NewBuyOffer => {
          contract
            .events()
            .new_buy_offer()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::BuyOfferWithdrawn => {
          contract
            .events()
            .buy_offer_withdrawn()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::Sale => {
          contract
            .events()
            .sale()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::RoyaltiesPaid => {
          contract
            .events()
            .royalties_paid()
            .stream()
            .for_each(dump::stream)
            .await
        }
      }
    } else {
      match self {
        Self::All => dump::query(contract.all_events().query().await?),
        Self::NewSellOffer => dump::query(contract.events().new_sell_offer().query().await?),
        Self::SellOfferWithdrawn => {
          dump::query(contract.events().sell_offer_withdrawn().query().await?)
        }
        Self::NewBuyOffer => dump::query(contract.events().new_buy_offer().query().await?),
        Self::BuyOfferWithdrawn => {
          dump::query(contract.events().buy_offer_withdrawn().query().await?)
        }
        Self::Sale => dump::query(contract.events().sale().query().await?),
        Self::RoyaltiesPaid => dump::query(contract.events().royalties_paid().query().await?),
      }
    }

    Ok(())
  }
}
