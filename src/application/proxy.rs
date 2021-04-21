use crate::{
  middleware::{
    ethereum::{prelude::*, *},
    *,
  },
  WrappedResult,
};
use tide::Server;
use web3::transports::WebSocket;

pub struct Config {
  pub address_header: HeaderName,
  pub backend: Url,
  pub balance_header: HeaderName,
  pub balance_requirement: Option<BalanceRequirement>,
  pub balance_scale: Option<BalanceScale>,
  pub challenge: Vec<u8>,
  pub provides_account_verification: bool,
  pub provides_balances: bool,
  pub provides_signatures: bool,
  pub web3_rpc_url: Url,
  pub secret_key: Option<SecretKey>,
  pub signature_header: HeaderName,
}

pub async fn server(config: Config) -> WrappedResult<Server<()>> {
  let mut server = tide::new();
  server.with(ProvidesForwardedHeader);

  if config.provides_signatures {
    server.with(ProvidesSignature {
      signature_header: config.signature_header.clone(),
      secret_key: config.secret_key.unwrap(),
      web3: Web3::new(WebSocket::new(config.web3_rpc_url.as_str()).await?),
      challenge: config.challenge.clone(),
    });
  }

  if config.provides_account_verification {
    server.with(ProvidesAccountVerification {
      signature_header: config.signature_header.clone(),
      address_header: config.address_header.clone(),
      status_code: StatusCode::PaymentRequired,
      web3: Web3::new(WebSocket::new(config.web3_rpc_url.as_str()).await?),
      challenge: config.challenge.clone(),
    });
  }

  if config.provides_balances {
    server.with(ProvidesBalance {
      address_header: config.address_header.clone(),
      balance_header: config.balance_header.clone(),
      web3: Web3::new(WebSocket::new(config.web3_rpc_url.as_str()).await?),
    });
  }

  if let Some(requirement) = config.balance_requirement {
    server.with(
      RequiresBalance {
        header: config.balance_header.clone(),
        requirement,
      }
      .scale(config.balance_scale.unwrap_or(BalanceScale::Gwei)),
    );
  }

  server.with(Proxy::new(config.backend));

  Ok(server)
}
