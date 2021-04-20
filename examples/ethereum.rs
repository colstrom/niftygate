use sig_proxy::middleware::{ethereum::prelude::*, *};
use std::str::FromStr;
use tide::{http::Url, log};
use web3::transports::WebSocket;
type WrappedError = Box<dyn std::error::Error>;

#[async_std::main]
async fn main() -> std::result::Result<(), WrappedError> {
  log::with_level(log::LevelFilter::Debug);

  let secret_key_data =
    hex_literal::hex!("4bfbe335b56dd7aee5e982b9aba0de4cf3495055b7dfdf538a9ab10ca028035f");
  std::fs::write("ethereum.key", secret_key_data)?;
  let secret_key_data = std::fs::read("ethereum.key")?;

  let transport_url = Url::parse("ws://127.0.0.1:7545")?;
  let transport = WebSocket::new(transport_url.as_str()).await?;

  let mut server = tide::new();
  server
    .with(ProvidesForwardedHeader)
    .with(ethereum::ProvidesSignature {
      signature_header: HeaderName::from_string(String::from("X-Web3-Signature"))?,
      secret_key: SecretKey::from_slice(&secret_key_data)?,
      web3: Web3::new(transport.clone()),
      message: b"license".to_vec(),
    })
    .with(ethereum::ProvidesAccountVerification {
      signature_header: HeaderName::from_string(String::from("X-Web3-Signature"))?,
      account_header: HeaderName::from_string(String::from("X-Web3-Account"))?,
      status_code: StatusCode::PaymentRequired,
      web3: Web3::new(transport.clone()),
      message: b"license".to_vec(),
    })
    .with(ethereum::ProvidesBalance {
      account_header: HeaderName::from_string(String::from("X-Web3-Account"))?,
      balance_header: HeaderName::from_string(String::from("X-Web3-Balance"))?,
      web3: Web3::new(transport.clone()),
    })
    .with(
      ethereum::RequiresBalance {
        header: HeaderName::from_string(String::from("X-Web3-Balance"))?,
        required: ethereum::BalanceRequirement::AtLeast(U256::from_str("5")?),
      }
      .scale(ethereum::BalanceUnit::Gwei),
    )
    .with(proxy::Proxy::new(Url::parse("http://127.0.0.1:8000")?));
  server.listen("127.0.0.1:8002").await?;

  Ok(())
}
