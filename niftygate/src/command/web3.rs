use crate::{
  middleware::ethereum::{BalanceRequirement, BalanceScale},
  HexData,
};
use anyhow::Result;
use secp256k1::SecretKey;
use std::{fs, path::PathBuf};
use structopt::StructOpt;
use tide::{
  http::{headers::HeaderName, Url},
  log,
};
use tide_rustls::TlsListener;
use web3::types::{Address, U256};

const ZERO_ADDRESS: [u8; 20] = [0; 20];
// const PERSONAL_SIGN_PREFIX: &str = "\x19Ethereum Signed Message:\n";

#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the proxy service")]
pub struct Command {
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

  #[structopt(
    env,
    long,
    takes_value = false,
    requires = "tls-certificate-path",
    requires = "tls-key-path"
  )]
  with_tls: bool,

  #[structopt(env, long, value_name = "Path", requires = "tls-key-path")]
  tls_certificate_path: Option<PathBuf>,

  #[structopt(env, long, value_name = "Path", requires = "tls-certificate-path")]
  tls_key_path: Option<PathBuf>,
}

impl Command {
  pub async fn execute(self) -> Result<()> {
    let secret_key = match (self.secret_key_data, self.secret_key_file) {
      (Some(data), _) => Some(SecretKey::from_slice(&data.0)?),
      (None, Some(path)) => Some(SecretKey::from_slice(&fs::read(path)?)?),
      (None, None) => None,
    };

    let balance_requirement = balance_requirement_from(self.balance_minimum, self.balance_maximum);

    let zero_address = Address::from_slice(&ZERO_ADDRESS);

    let erc1155 = crate::application::proxy::ERC1155Config {
      balance_header: self.erc1155_balance_header,
      balance_requirement: balance_requirement_from(
        self.erc1155_balance_minimum,
        self.erc1155_balance_maximum,
      ),
      contract_address: match self.erc1155_contract_address {
        Some(address) => address,
        None => zero_address,
      },
      provides_balances: self.provides_erc1155_balance,
    };

    let erc20 = crate::application::proxy::ERC20Config {
      balance_header: self.erc20_balance_header,
      balance_requirement: balance_requirement_from(
        self.erc20_balance_minimum,
        self.erc20_balance_maximum,
      ),
      contract_address: match self.erc20_contract_address {
        Some(address) => address,
        None => zero_address,
      },
      name_header: self.erc20_name_header,
      provides_balances: self.provides_erc20_balance,
      provides_name: self.provides_erc20_name,
      provides_symbol: self.provides_erc20_symbol,
      symbol_header: self.erc20_symbol_header,
    };

    let erc721 = crate::application::proxy::ERC721Config {
      balance_header: self.erc721_balance_header,
      balance_requirement: balance_requirement_from(
        self.erc721_balance_minimum,
        self.erc721_balance_maximum,
      ),
      contract_address: match self.erc721_contract_address {
        Some(address) => address,
        None => zero_address,
      },
      name_header: self.erc721_name_header,
      provides_balances: self.provides_erc721_balance,
      provides_name: self.provides_erc721_name,
      provides_symbol: self.provides_erc721_symbol,
      symbol_header: self.erc721_symbol_header,
    };

    let erc777 = crate::application::proxy::ERC777Config {
      balance_header: self.erc777_balance_header,
      balance_requirement: balance_requirement_from(
        self.erc777_balance_minimum,
        self.erc777_balance_maximum,
      ),
      contract_address: match self.erc777_contract_address {
        Some(address) => address,
        None => zero_address,
      },
      name_header: self.erc777_name_header,
      provides_balances: self.provides_erc777_balance,
      provides_name: self.provides_erc777_name,
      provides_symbol: self.provides_erc777_symbol,
      symbol_header: self.erc777_symbol_header,
    };

    // let mut challenge = String::from(PERSONAL_SIGN_PREFIX);
    // challenge.push_str(&self.challenge);

    let challenge = self.challenge;

    let config = crate::application::proxy::Config {
      address_header: self.address_header,
      backend: self.backend,
      balance_header: self.balance_header,
      balance_requirement,
      balance_scale: Some(self.balance_scale),
      challenge: challenge.as_bytes().to_vec(),
      erc1155,
      erc20,
      erc721,
      erc777,
      provides_account_verification: self.provides_account_verification,
      provides_balances: self.provides_balances,
      provides_signatures: self.provides_signatures,
      secret_key,
      signature_header: self.signature_header,
      web3_rpc_url: self.web3_rpc_url,
    };

    log::with_level(log::LevelFilter::Debug);

    let server = crate::application::proxy::server(config).await?;

    if self.with_tls {
      match (self.tls_certificate_path, self.tls_key_path) {
        (Some(tls_certificate_path), Some(tls_key_path)) => {
          server.listen(TlsListener::build()
            .addrs(&self.listen)
            .cert(&tls_certificate_path)
            .key(&tls_key_path)).await?
          },
          _ => panic!("Missing either certificate or key. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻"),
        }
    } else {
      server.listen(&self.listen).await?
    }

    Ok(())
  }
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
