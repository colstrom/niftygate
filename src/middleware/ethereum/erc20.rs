pub mod prelude {
  pub use crate::openzeppelin::contracts::token::erc20::ERC20;
  pub use ethcontract::web3::{
    transports::WebSocket,
    types::{Address, U256},
    Web3,
  };
  pub use tide::http::{
    headers::{HeaderName, HeaderValue},
    StatusCode, Url,
  };
}

use prelude::*;

use std::{result, str::FromStr};
use tide::{utils::async_trait, Middleware, Next, Request, Response, Result};

#[derive(Clone)]
pub struct ProvidesERC20Balance {
  pub address_header: HeaderName,
  pub balance_header: HeaderName,
  pub name_header: Option<HeaderName>,
  pub symbol_header: Option<HeaderName>,
  pub contract: ERC20,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ProvidesERC20Balance {
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

    if let Some(name) = &self.name_header {
      if let Ok(values) = self.contract.name().call().await {
        request.append_header(name, values)
      }
    }

    if let Some(name) = &self.symbol_header {
      if let Ok(values) = self.contract.symbol().call().await {
        request.append_header(name, values)
      }
    }

    for account in addresses {
      match self.contract.balance_of(account).call().await {
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
