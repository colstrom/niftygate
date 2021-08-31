use ethcontract::{dyns::DynTransport, web3::transports::WebSocket};
use niftygate::{
  middleware::{
    ethereum::{prelude::*, *},
    *,
  },
  WrappedResult,
};
use std::str::FromStr;
use tide::{http::Url, log};

#[async_std::main]
async fn main() -> WrappedResult<()> {
  log::with_level(log::LevelFilter::Debug);

  let secret_key_data = std::env::var("SECRET_KEY_DATA")?.as_bytes().to_vec();
  let rpc_url = Url::parse("ws://127.0.0.1:7545")?;

  let inner = WebSocket::new(rpc_url.as_str()).await?;
  let transport = DynTransport::new(inner);
  let web3 = Web3::new(transport);

  let mut server = tide::new();
  server
    .with(ProvidesForwardedHeader)
    .with(ProvidesSignature {
      signature_header: HeaderName::from_string(String::from("X-Web3-Signature"))?,
      secret_key: SecretKey::from_slice(&secret_key_data)?,
      web3: web3.clone(),
      challenge: b"totes-legit".to_vec(),
    })
    .with(ProvidesAccountVerification {
      signature_header: HeaderName::from_string(String::from("X-Web3-Signature"))?,
      address_header: HeaderName::from_string(String::from("X-Web3-Account-Address"))?,
      status_code: StatusCode::PaymentRequired,
      web3: web3.clone(),
      challenge: b"totes-legit".to_vec(),
    })
    .with(ProvidesBalance {
      address_header: HeaderName::from_string(String::from("X-Web3-Account-Address"))?,
      balance_header: HeaderName::from_string(String::from("X-Web3-Account-Balance"))?,
      web3: web3.clone(),
    })
    .with(
      RequiresBalance {
        header: HeaderName::from_string(String::from("X-Web3-Account-Balance"))?,
        requirement: BalanceRequirement::AtLeast(U256::from_str("5")?),
      }
      .scale(BalanceScale::Gwei),
    )
    .with(proxy::Proxy::new(Url::parse("http://127.0.0.1:8000")?));
  server.listen("127.0.0.1:8002").await?;

  Ok(())
}
