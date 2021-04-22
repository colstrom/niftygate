use crate::{middleware::ethereum::*, WrappedResult};
use secp256k1::SecretKey;
use std::{fs, path::PathBuf, str::FromStr};
use structopt::StructOpt;
use strum::VariantNames;
use tide::{
  http::{headers::HeaderName, Url},
  log,
};
use web3::types::U256;

#[derive(Debug)]
pub struct HexData(Vec<u8>);

impl FromStr for HexData {
  type Err = hex::FromHexError;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    hex::decode(s).map(Self)
  }
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub enum Options {
  #[structopt(about = "Runs a sample app that returns request headers in the response body")]
  Demo {
    #[structopt(
      env,
      long,
      short,
      value_name = "address",
      default_value = "127.0.0.1:8080"
    )]
    listen: String,
  },
  #[structopt(about = "Prints a table of recognized units and scaling values")]
  Units {},
  #[structopt(about = "Runs the proxy service")]
  Web3 {
    #[structopt(
      env,
      long,
      short,
      value_name = "address",
      default_value = "0.0.0.0:8000"
    )]
    listen: String,

    #[structopt(
      env,
      long,
      short,
      value_name = "url",
      default_value = "http://127.0.0.1:8080"
    )]
    backend: Url,

    #[structopt(
      env,
      short,
      long,
      value_name = "url",
      default_value = "ws://127.0.0.1:7545"
    )]
    web3_rpc_url: Url,

    #[structopt(
      env,
      long,
      value_name = "name",
      default_value = "X-Web3-Account-Address"
    )]
    address_header: HeaderName,

    #[structopt(
      env,
      long,
      value_name = "name",
      default_value = "X-Web3-Account-Balance"
    )]
    balance_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-Signature")]
    signature_header: HeaderName,

    #[structopt(env, long, short = "k", value_name = "path")]
    secret_key_file: Option<PathBuf>,

    #[structopt(env, long, short = "K", value_name = "hex")]
    secret_key_data: Option<HexData>,

    #[structopt(env, long, short, value_name = "phrase", default_value = "totes-legit")]
    challenge: String,

    #[structopt(env, long, short = "u", value_name = "unit", default_value = "Wei")]
    balance_scale: BalanceScale,

    #[structopt(env, long, value_name = "amount")]
    balance_minimum: Option<U256>,

    #[structopt(env, long, value_name = "amount")]
    balance_maximum: Option<U256>,

    #[structopt(
      env,
      long,
      short = "S",
      takes_value = false,
      help = "provide signatures"
    )]
    provides_signatures: bool,

    #[structopt(
      env,
      long,
      short = "V",
      takes_value = false,
      help = "verify signatures and provide account addresses"
    )]
    provides_account_verification: bool,

    #[structopt(
      env,
      long,
      short = "B",
      takes_value = false,
      help = "provide account balances"
    )]
    provides_balances: bool,
  },
}

pub async fn run() -> WrappedResult<()> {
  match Options::from_args() {
    Options::Demo { listen } => {
      log::with_level(log::LevelFilter::Debug);
      crate::application::demo::server().listen(listen).await?;
    }
    Options::Units { .. } => {
      if let Some(max) = BalanceScale::VARIANTS.iter().map(|&s| s.len()).max() {
        for &variant in BalanceScale::VARIANTS {
          let scale = BalanceScale::from_str(&variant)?.scale();
          println!("{:<pad$} => {}", &variant, scale, pad = max);
        }
      };
    }
    Options::Web3 {
      address_header,
      backend,
      balance_header,
      balance_maximum,
      balance_minimum,
      balance_scale: balance_unit,
      listen,
      challenge: message,
      secret_key_data,
      secret_key_file,
      signature_header,
      web3_rpc_url,
      provides_account_verification,
      provides_balances,
      provides_signatures,
      ..
    } => {
      let secret_key = match (secret_key_data, secret_key_file) {
        (Some(data), _) => Some(SecretKey::from_slice(&data.0)?),
        (None, Some(path)) => Some(SecretKey::from_slice(&fs::read(path)?)?),
        (None, None) => None,
      };

      let balance_requirement = match (balance_minimum, balance_maximum) {
        (Some(min), None) => Some(BalanceRequirement::AtLeast(min)),
        (None, Some(max)) => Some(BalanceRequirement::AtMost(max)),
        (Some(min), Some(max)) => Some(BalanceRequirement::Between(min, max)),
        (None, None) => None,
      };

      let config = crate::application::proxy::Config {
        address_header,
        backend,
        balance_header,
        balance_requirement,
        balance_scale: Some(balance_unit),
        challenge: message.as_bytes().to_vec(),
        provides_account_verification,
        provides_balances,
        provides_signatures,
        web3_rpc_url,
        secret_key,
        signature_header,
      };

      log::with_level(log::LevelFilter::Debug);
      crate::application::proxy::server(config)
        .await?
        .listen(listen)
        .await?;
    }
  }

  Ok(())
}
