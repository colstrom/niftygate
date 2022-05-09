use super::super::{dump, CallReturn, SendReturn};
use anyhow::Result;
use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  futures::StreamExt,
  Address, U256,
};
use humantime::{Duration, Timestamp};
use niftygate_bindings::openzeppelin::contracts_legacy::example::sample_crowdsale::Contract;
use std::time::UNIX_EPOCH;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  // #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
  #[structopt(long, value_name = "RFC3339")]
  opening_time: Timestamp,
  // #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
  #[structopt(long, value_name = "RFC3339")]
  closing_time: Option<Timestamp>,
  #[structopt(long, value_name = "Timespan")]
  duration: Option<Duration>,
  #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
  rate: U256,
  #[structopt(long, value_name = "H160")]
  wallet: Address,
  #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
  cap: U256,
  #[structopt(long, value_name = "ERC20Mintable")]
  token: Address,
  #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
  goal: U256,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> Result<DynDeployBuilder<Contract>> {
    let opening_time = U256::from(self.opening_time.duration_since(UNIX_EPOCH)?.as_secs());
    let closing_time = if let Some(closing_time) = self.closing_time {
      U256::from(closing_time.duration_since(UNIX_EPOCH)?.as_secs())
    } else if let Some(duration) = self.duration {
      opening_time + U256::from(duration.as_secs())
    } else {
      panic!("Missing either closing time or duration. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻")
    };

    dbg!(&opening_time, &closing_time);
    let builder = Contract::builder(
      web3,
      opening_time,
      closing_time,
      self.rate,
      self.wallet,
      self.cap,
      self.token,
      self.goal,
    );

    Ok(builder)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum CallCommand {
  Token,       // Crowdsale
  Wallet,      // Crowdsale
  Rate,        // Crowdsale
  WeiRaised,   // Crowdsale
  Cap,         // CappedCrowdsale is Crowdsale
  CapReached,  // CappedCrowdsale is Crowdsale
  OpeningTime, // TimedCrowdsale is Crowdsale
  ClosingTime, // TimedCrowdsale is Crowdsale
  IsOpen,      // TimedCrowdsale is Crowdsale
  HasClosed,   // TimedCrowdsale is Crowdsale
  Finalized,   // FinalizableCrowdsale is TimedCrowdsale
  Goal,        // RefundableCrowdsale is FinalizableCrowdsale
  GoalReached, // RefundableCrowdsale is FinalizableCrowdsale
}

impl CallCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::Token => contract.token().into(),
      Self::Wallet => contract.wallet().into(),
      Self::Rate => contract.rate().into(),
      Self::WeiRaised => contract.wei_raised().into(),

      Self::Cap => contract.cap().into(),
      Self::CapReached => contract.cap_reached().into(),

      Self::OpeningTime => contract.opening_time().into(),
      Self::ClosingTime => contract.closing_time().into(),
      Self::IsOpen => contract.is_open().into(),
      Self::HasClosed => contract.has_closed().into(),
      Self::Finalized => contract.finalized().into(),

      Self::Goal => contract.goal().into(),
      Self::GoalReached => contract.goal_reached().into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  BuyTokens {
    // Crowdsale
    #[structopt(long, value_name = "H160")]
    beneficiary: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    value: U256,
  },
  Finalize, // FinalizableCrowdsale is TimedCrowdsale
  ClaimRefund {
    // RefundableCrowdsale is FinalizableCrowdsale
    #[structopt(long, value_name = "H160")]
    refundee: Address,
  },
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::BuyTokens { beneficiary, value } => contract.buy_tokens(beneficiary).value(value).into(),
      Self::Finalize => contract.finalize().into(),
      Self::ClaimRefund { refundee } => contract.claim_refund(refundee).into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum EventsCommand {
  All,
  TokensPurchased,        // Crowdsale
  TimedCrowdsaleExtended, // TimedCrowdsale is Crowdsale
  CrowdsaleFinalized,     // FinalizableCrowdsale is TimedCrowdSale
}

impl EventsCommand {
  pub async fn execute(self, web3: &DynWeb3, address: Address, stream: bool) -> Result<()> {
    let contract = Contract::at(web3, address);

    if stream {
      match self {
        Self::All => contract.all_events().stream().for_each(dump::stream).await,
        Self::TokensPurchased => {
          contract
            .events()
            .tokens_purchased()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::TimedCrowdsaleExtended => {
          contract
            .events()
            .timed_crowdsale_extended()
            .stream()
            .for_each(dump::stream)
            .await
        }
        Self::CrowdsaleFinalized => {
          contract
            .events()
            .crowdsale_finalized()
            .stream()
            .for_each(dump::stream)
            .await
        }
      }
    } else {
      match self {
        Self::All => dump::query(contract.all_events().query().await?),
        Self::TokensPurchased => dump::query(contract.events().tokens_purchased().query().await?),
        Self::TimedCrowdsaleExtended => {
          dump::query(contract.events().timed_crowdsale_extended().query().await?)
        }
        Self::CrowdsaleFinalized => {
          dump::query(contract.events().crowdsale_finalized().query().await?)
        }
      }
    }

    Ok(())
  }
}
