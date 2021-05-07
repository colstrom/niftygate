pub mod prelude {
  pub use ethcontract::web3::{
    transports::WebSocket,
    types::{Address, BlockNumber, U256},
    Web3,
  };
  pub use tide::http::{
    headers::{HeaderName, HeaderValue},
    StatusCode, Url,
  };
}

use prelude::*;

use std::{result, str::FromStr};
use strum::{AsRefStr, EnumString, EnumVariantNames};
use tide::{utils::async_trait, Middleware, Next, Request, Response, Result};

#[derive(AsRefStr, Clone, Debug, EnumString, EnumVariantNames)]
pub enum BalanceScale {
  Wei,
  Kwei,
  Babbage,
  Mwei,
  Lovelace,
  Gwei,
  Shannon,
  Twei,
  Szabo,
  Pwei,
  Finney,
  Ether,
  Buterin,
}

impl BalanceScale {
  pub fn scale(&self) -> U256 {
    match self {
      BalanceScale::Wei => U256::exp10(0),
      BalanceScale::Kwei | BalanceScale::Babbage => U256::exp10(3),
      BalanceScale::Mwei | BalanceScale::Lovelace => U256::exp10(6),
      BalanceScale::Gwei | BalanceScale::Shannon => U256::exp10(9),
      BalanceScale::Twei | BalanceScale::Szabo => U256::exp10(12),
      BalanceScale::Pwei | BalanceScale::Finney => U256::exp10(15),
      BalanceScale::Ether | BalanceScale::Buterin => U256::exp10(18),
    }
  }
}

#[derive(Clone, Debug)]
pub enum BalanceRequirement {
  AtLeast(U256),
  AtMost(U256),
  Between(U256, U256),
}

#[derive(Clone)]
pub struct ProvidesBalance {
  pub address_header: HeaderName,
  pub balance_header: HeaderName,
  pub web3: Web3<WebSocket>,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ProvidesBalance {
  async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
    let addresses = match request.header(&self.address_header) {
      None => return Ok(Response::new(StatusCode::NetworkAuthenticationRequired)),
      Some(header_values) => match header_values
        .into_iter()
        .map(|input| hex::decode(input.as_str()))
        .collect::<result::Result<Vec<Vec<u8>>, hex::FromHexError>>()
      {
        Err(_) => return Ok(Response::new(StatusCode::BadRequest)),
        Ok(raw_addresses) => raw_addresses
          .into_iter()
          .map(|src| Address::from_slice(&src))
          .collect::<Vec<Address>>(),
      },
    };

    let block = match self.web3.eth().block_number().await {
      Err(_) => return Ok(Response::new(StatusCode::InternalServerError)),
      Ok(block) => BlockNumber::Number(block),
    };

    for address in addresses {
      match self.web3.eth().balance(address, Some(block)).await {
        // match self.web3.eth().balance(address, None).await {
        Err(_) => return Ok(Response::new(StatusCode::InternalServerError)),
        Ok(balance) => match HeaderValue::from_str(&balance.to_string()) {
          Err(_) => return Ok(Response::new(StatusCode::InternalServerError)),
          Ok(value) => request.append_header(&self.balance_header, value),
        },
      }
    }

    Ok(next.run(request).await)
  }
}

#[derive(Clone)]
pub struct RequiresBalance {
  pub header: HeaderName,
  pub requirement: BalanceRequirement,
}

impl RequiresBalance {
  pub fn scale(mut self, unit: BalanceScale) -> Self {
    let scale = unit.scale();
    let new = match self.requirement {
      BalanceRequirement::AtLeast(min) => {
        BalanceRequirement::AtLeast(U256::saturating_mul(min, scale))
      }
      BalanceRequirement::AtMost(max) => {
        BalanceRequirement::AtMost(U256::saturating_mul(max, scale))
      }
      BalanceRequirement::Between(min, max) => BalanceRequirement::Between(
        U256::saturating_mul(min, scale),
        U256::saturating_mul(max, scale),
      ),
    };

    self.requirement = new;
    self
  }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequiresBalance {
  async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> Result {
    match request.header(&self.header) {
      None => return Ok(Response::new(StatusCode::BadRequest)),
      Some(header_values) => {
        match header_values
          .into_iter()
          .map(|value| U256::from_str(value.as_str()))
          .collect::<result::Result<Vec<U256>, _>>()
        {
          Err(_) => return Ok(Response::new(StatusCode::BadRequest)),
          Ok(balances) => {
            if balances.into_iter().any(|balance| match self.requirement {
              BalanceRequirement::AtLeast(min) => balance.ge(&min),
              BalanceRequirement::AtMost(max) => balance.le(&max),
              BalanceRequirement::Between(min, max) => balance.ge(&min) && balance.le(&max),
            }) {
              println!("Balance meets requirement of {:?}", self.requirement);
              return Ok(next.run(request).await);
            } else {
              return Ok(Response::new(StatusCode::PaymentRequired));
            }
          }
        }
      }
    }
  }
}
