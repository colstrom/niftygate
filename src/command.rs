use crate::{middleware::ethereum::*, WrappedResult};
use secp256k1::SecretKey;
use std::{fs, path::PathBuf, str::FromStr};
use structopt::StructOpt;
use strum::VariantNames;
use tide::{
  http::{headers::HeaderName, Url},
  log,
};
use web3::types::{Address, U256};

mod contract;
mod guide;

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

    #[structopt(
      env,
      long,
      value_name = "name",
      default_value = "X-Web3-ERC1155-Balance"
    )]
    erc1155_balance_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC20-Balance")]
    erc20_balance_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC20-Name")]
    erc20_name_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC20-Symbol")]
    erc20_symbol_header: HeaderName,

    #[structopt(
      env,
      long,
      value_name = "name",
      default_value = "X-Web3-ERC721-Balance"
    )]
    erc721_balance_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC721-Name")]
    erc721_name_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC721-Symbol")]
    erc721_symbol_header: HeaderName,

    #[structopt(
      env,
      long,
      value_name = "name",
      default_value = "X-Web3-ERC777-Balance"
    )]
    erc777_balance_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC777-Name")]
    erc777_name_header: HeaderName,

    #[structopt(env, long, value_name = "name", default_value = "X-Web3-ERC777-Symbol")]
    erc777_symbol_header: HeaderName,

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

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str))]
    balance_minimum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str))]
    balance_maximum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc1155_balance_minimum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc1155_balance_maximum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc20_balance_minimum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc20_balance_maximum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc721_balance_minimum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc721_balance_maximum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc777_balance_minimum: Option<U256>,

    #[structopt(env, long, value_name = "amount", parse(try_from_str = U256::from_dec_str), )]
    erc777_balance_maximum: Option<U256>,

    #[structopt(env, long, value_name = "address")]
    erc1155_contract_address: Option<Address>,

    #[structopt(env, long, value_name = "address")]
    erc20_contract_address: Option<Address>,

    #[structopt(env, long, value_name = "address")]
    erc721_contract_address: Option<Address>,

    #[structopt(env, long, value_name = "address")]
    erc777_contract_address: Option<Address>,

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

    #[structopt(env, long, takes_value = false, help = "provide ERC1155 balances")]
    provides_erc1155_balance: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC20 balances")]
    provides_erc20_balance: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC20 names")]
    provides_erc20_name: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC20 symbols")]
    provides_erc20_symbol: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC721 balances")]
    provides_erc721_balance: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC721 names")]
    provides_erc721_name: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC721 symbols")]
    provides_erc721_symbol: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC777 balances")]
    provides_erc777_balance: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC777 names")]
    provides_erc777_name: bool,

    #[structopt(env, long, takes_value = false, help = "provide ERC777 symbols")]
    provides_erc777_symbol: bool,
  },
  Contract(contract::Command),
  Guide(guide::Command),
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
    Options::Contract(command) => command.execute().await?,
    Options::Guide(command) => command.execute()?,
    Options::Web3 {
      address_header,
      backend,
      balance_header,
      balance_maximum,
      balance_minimum,
      balance_scale,
      challenge: message,
      erc1155_balance_header,
      erc1155_balance_maximum,
      erc1155_balance_minimum,
      erc1155_contract_address,
      erc20_balance_header,
      erc20_balance_maximum,
      erc20_balance_minimum,
      erc20_contract_address,
      erc20_name_header,
      erc20_symbol_header,
      erc721_balance_header,
      erc721_balance_maximum,
      erc721_balance_minimum,
      erc721_contract_address,
      erc721_name_header,
      erc721_symbol_header,
      erc777_balance_header,
      erc777_balance_maximum,
      erc777_balance_minimum,
      erc777_contract_address,
      erc777_name_header,
      erc777_symbol_header,
      listen,
      provides_account_verification,
      provides_balances,
      provides_erc1155_balance,
      provides_erc20_balance,
      provides_erc20_name,
      provides_erc20_symbol,
      provides_erc721_balance,
      provides_erc721_name,
      provides_erc721_symbol,
      provides_erc777_balance,
      provides_erc777_name,
      provides_erc777_symbol,
      provides_signatures,
      secret_key_data,
      secret_key_file,
      signature_header,
      web3_rpc_url,
      ..
    } => {
      let secret_key = match (secret_key_data, secret_key_file) {
        (Some(data), _) => Some(SecretKey::from_slice(&data.0)?),
        (None, Some(path)) => Some(SecretKey::from_slice(&fs::read(path)?)?),
        (None, None) => None,
      };

      let balance_requirement = balance_requirement_from(balance_minimum, balance_maximum);

      let zero_address = Address::from_str("0x0000000000000000000000000000000000000000")?;

      let erc1155 = crate::application::proxy::ERC1155Config {
        balance_header: erc1155_balance_header,
        balance_requirement: balance_requirement_from(
          erc1155_balance_minimum,
          erc1155_balance_maximum,
        ),
        contract_address: match erc1155_contract_address {
          Some(address) => address,
          None => zero_address.clone(),
        },
        provides_balances: provides_erc1155_balance,
      };

      let erc20 = crate::application::proxy::ERC20Config {
        balance_header: erc20_balance_header,
        balance_requirement: balance_requirement_from(erc20_balance_minimum, erc20_balance_maximum),
        contract_address: match erc20_contract_address {
          Some(address) => address,
          None => zero_address.clone(),
        },
        name_header: erc20_name_header,
        provides_balances: provides_erc20_balance,
        provides_name: provides_erc20_name,
        provides_symbol: provides_erc20_symbol,
        symbol_header: erc20_symbol_header,
      };

      let erc721 = crate::application::proxy::ERC721Config {
        balance_header: erc721_balance_header,
        balance_requirement: balance_requirement_from(
          erc721_balance_minimum,
          erc721_balance_maximum,
        ),
        contract_address: match erc721_contract_address {
          Some(address) => address,
          None => zero_address.clone(),
        },
        name_header: erc721_name_header,
        provides_balances: provides_erc721_balance,
        provides_name: provides_erc721_name,
        provides_symbol: provides_erc721_symbol,
        symbol_header: erc721_symbol_header,
      };

      let erc777 = crate::application::proxy::ERC777Config {
        balance_header: erc777_balance_header,
        balance_requirement: balance_requirement_from(
          erc777_balance_minimum,
          erc777_balance_maximum,
        ),
        contract_address: match erc777_contract_address {
          Some(address) => address,
          None => zero_address.clone(),
        },
        name_header: erc777_name_header,
        provides_balances: provides_erc777_balance,
        provides_name: provides_erc777_name,
        provides_symbol: provides_erc777_symbol,
        symbol_header: erc777_symbol_header,
      };

      let config = crate::application::proxy::Config {
        address_header,
        backend,
        balance_header,
        balance_requirement,
        balance_scale: Some(balance_scale),
        challenge: message.as_bytes().to_vec(),
        erc1155,
        erc20,
        erc721,
        erc777,
        provides_account_verification,
        provides_balances,
        provides_signatures,
        secret_key,
        signature_header,
        web3_rpc_url,
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

fn balance_requirement_from(
  balance_minimum: Option<U256>,
  balance_maximum: Option<U256>,
) -> Option<BalanceRequirement> {
  match (balance_minimum, balance_maximum) {
    (Some(min), None) => Some(BalanceRequirement::AtLeast(min)),
    (None, Some(max)) => Some(BalanceRequirement::AtMost(max)),
    (Some(min), Some(max)) => Some(BalanceRequirement::Between(min, max)),
    (None, None) => None,
  }
}
