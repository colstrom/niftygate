use crate::WrappedResult;
use async_std::println;
use ethcontract::{
  dyns::{DynMethodBuilder, DynTransport, DynViewMethodBuilder, DynWeb3},
  transaction::TransactionResult,
  web3::transports::WebSocket,
  Account, Address, Password, PrivateKey, Void, Web3, U256,
};
use structopt::StructOpt;
use tide::http::Url;

mod erc1155_preset_minter_pauser;
mod erc20_preset_fixed_supply;
mod erc20_preset_minter_pauser;
mod erc721_preset_minter_pauser_auto_id;
mod erc777_preset_fixed_supply;

#[derive(Debug, StructOpt)]
#[structopt(about = "Utilities for dealing with Smart Contracts")]
pub struct Command {
  #[structopt(
    env,
    short,
    long,
    value_name = "url",
    default_value = "ws://127.0.0.1:7545"
  )]
  web3_rpc_url: Url,

  #[structopt(long, value_name = "H160", required_unless = "private-key")]
  from: Option<Address>,

  #[structopt(long, value_name = "String", conflicts_with = "private-key")]
  password: Option<String>,

  #[structopt(env, long, value_name = "HexData", conflicts_with = "password")]
  private_key: Option<PrivateKey>,

  #[structopt(long, value_name = "U64", conflicts_with = "from")]
  chain_id: Option<u64>,

  #[structopt(subcommand)]
  variant: CommandVariant,
}
#[derive(Debug, StructOpt)]
#[structopt(about = "Utilities for dealing with Smart Contracts")]
pub enum CommandVariant {
  Deploy(DeployCommand),
  Call(CallCommand),
  Send(SendCommand),
}

impl Command {
  pub async fn execute(self) -> WrappedResult<()> {
    let account = match self.private_key {
      Some(private_key) => Account::Offline(private_key, self.chain_id),
      None => match self.from {
        Some(address) => match self.password {
          Some(password) => Account::Locked(address, Password::new(password), None),
          None => Account::Local(address, None),
        },
        None => panic!("Missing either address or private key. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻"),
      },
    };

    let url = self.web3_rpc_url.as_str();
    let inner = WebSocket::new(url).await?;
    let transport = DynTransport::new(inner);
    let web3 = Web3::new(transport);

    match self.variant {
      CommandVariant::Deploy(variant) => variant.execute(&web3, account).await,
      CommandVariant::Call(variant) => variant.execute(&web3, account).await,
      CommandVariant::Send(variant) => variant.execute(&web3, account).await,
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deploys a contract, returns Contract Address.")]
pub struct DeployCommand {
  #[structopt(subcommand)]
  variant: DeployVariant,
}

impl DeployCommand {
  pub async fn execute(self, web3: &DynWeb3, account: Account) -> WrappedResult<()> {
    let address = match self.variant {
      DeployVariant::ERC1155PresetMinterPauser(variant) => {
        variant.build(web3).from(account).deploy().await?.address()
      }
      DeployVariant::ERC20PresetFixedSupply(variant) => {
        variant.build(web3).from(account).deploy().await?.address()
      }
      DeployVariant::ERC20PresetMinterPauser(variant) => {
        variant.build(web3).from(account).deploy().await?.address()
      }
      DeployVariant::ERC721PresetMinterPauserAutoId(variant) => {
        variant.build(web3).from(account).deploy().await?.address()
      }
      DeployVariant::ERC777PresetFixedSupply(variant) => {
        variant.build(web3).from(account).deploy().await?.address()
      }
    };

    println!("Deployed at {:?}", address).await;

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum DeployVariant {
  ERC1155PresetMinterPauser(erc1155_preset_minter_pauser::DeployCommand),
  ERC20PresetFixedSupply(erc20_preset_fixed_supply::DeployCommand),
  ERC20PresetMinterPauser(erc20_preset_minter_pauser::DeployCommand),
  ERC721PresetMinterPauserAutoId(erc721_preset_minter_pauser_auto_id::DeployCommand),
  ERC777PresetFixedSupply(erc777_preset_fixed_supply::DeployCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Calls a read-only method of a deployed contract, returns Value.")]
pub struct CallCommand {
  #[structopt(env, long, value_name = "H160")]
  pub(crate) contract_address: Address,

  #[structopt(subcommand)]
  variant: CallVariant,
}

impl CallCommand {
  pub async fn execute(self, web3: &DynWeb3, account: Account) -> WrappedResult<()> {
    let account = account.address();
    let address = self.contract_address;

    let callable = match self.variant {
      CallVariant::ERC1155PresetMinterPauser(variant) => variant.build(web3, address),
      CallVariant::ERC20PresetFixedSupply(variant) => variant.build(web3, address),
      CallVariant::ERC20PresetMinterPauser(variant) => variant.build(web3, address),
      CallVariant::ERC721PresetMinterPauserAutoId(variant) => variant.build(web3, address),
      CallVariant::ERC777PresetFixedSupply(variant) => variant.build(web3, address),
    };

    let result = match callable {
      CallReturn::Address(method) => method
        .from(account)
        .call()
        .await
        .and_then(|op| Ok(format!("{:?}", op)))?,
      CallReturn::Bool(method) => method.from(account).call().await?.to_string(),
      CallReturn::String(method) => method.from(account).call().await?.to_string(),
      CallReturn::U256(method) => method.from(account).call().await?.to_string(),
      CallReturn::U8(method) => method.from(account).call().await?.to_string(),
      CallReturn::VecOfAddress(method) => method
        .from(account)
        .call()
        .await?
        .into_iter()
        .map(|address| address.to_string())
        .collect::<Vec<String>>()
        .join("\n"),
      CallReturn::VecOfU256(method) => method
        .from(account)
        .call()
        .await?
        .into_iter()
        .map(|address| address.to_string())
        .collect::<Vec<String>>()
        .join("\n"),
      CallReturn::Void(method) => method
        .from(account)
        .call()
        .await
        .and_then(|op| Ok(format!("{:?}", op)))?,
    };

    println!("{}", result).await;

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum CallVariant {
  ERC1155PresetMinterPauser(erc1155_preset_minter_pauser::CallCommand),
  ERC20PresetFixedSupply(erc20_preset_fixed_supply::CallCommand),
  ERC20PresetMinterPauser(erc20_preset_minter_pauser::CallCommand),
  ERC721PresetMinterPauserAutoId(erc721_preset_minter_pauser_auto_id::CallCommand),
  ERC777PresetFixedSupply(erc777_preset_fixed_supply::CallCommand),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Sends a transaction to a deployed contract, returns Transaction Hash.")]
pub struct SendCommand {
  #[structopt(env, long, value_name = "H160")]
  pub(crate) contract_address: Address,

  #[structopt(subcommand)]
  variant: SendVariant,
}

impl SendCommand {
  pub async fn execute(self, web3: &DynWeb3, account: Account) -> WrappedResult<()> {
    let address = self.contract_address;

    let sendable = match self.variant {
      SendVariant::ERC1155PresetMinterPauser(variant) => variant.build(web3, address),
      SendVariant::ERC20PresetFixedSupply(variant) => variant.build(web3, address),
      SendVariant::ERC20PresetMinterPauser(variant) => variant.build(web3, address),
      SendVariant::ERC721PresetMinterPauserAutoId(variant) => variant.build(web3, address),
      SendVariant::ERC777PresetFixedSupply(variant) => variant.build(web3, address),
    };

    let result = match sendable {
      SendReturn::Bool(method) => method.from(account).send().await?,
      SendReturn::Void(method) => method.from(account).send().await?,
    };

    match result {
      TransactionResult::Hash(hash) => println!("Pending (Transaction {:?})", hash).await,
      TransactionResult::Receipt(receipt) => {
        let hash = receipt.transaction_hash;
        if let Some(status) = receipt.status {
          if status.is_zero() {
            println!("Failure (Transaction {:?})", hash).await
          } else {
            println!("Success (Transaction {:?})", hash).await
          }
        }
      }
    }

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "verbatim")]
pub enum SendVariant {
  ERC1155PresetMinterPauser(erc1155_preset_minter_pauser::SendCommand),
  ERC20PresetFixedSupply(erc20_preset_fixed_supply::SendCommand),
  ERC20PresetMinterPauser(erc20_preset_minter_pauser::SendCommand),
  ERC721PresetMinterPauserAutoId(erc721_preset_minter_pauser_auto_id::SendCommand),
  ERC777PresetFixedSupply(erc777_preset_fixed_supply::SendCommand),
}

pub enum CallReturn {
  Address(DynViewMethodBuilder<Address>),
  Bool(DynViewMethodBuilder<bool>),
  String(DynViewMethodBuilder<String>),
  U256(DynViewMethodBuilder<U256>),
  U8(DynViewMethodBuilder<u8>),
  VecOfAddress(DynViewMethodBuilder<Vec<Address>>),
  VecOfU256(DynViewMethodBuilder<Vec<U256>>),
  Void(DynViewMethodBuilder<Void>),
}

impl From<DynViewMethodBuilder<Address>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<Address>) -> Self {
    Self::Address(builder)
  }
}

impl From<DynViewMethodBuilder<bool>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<bool>) -> Self {
    Self::Bool(builder)
  }
}

impl From<DynViewMethodBuilder<String>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<String>) -> Self {
    Self::String(builder)
  }
}

impl From<DynViewMethodBuilder<U256>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<U256>) -> Self {
    Self::U256(builder)
  }
}

impl From<DynViewMethodBuilder<u8>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<u8>) -> Self {
    Self::U8(builder)
  }
}

impl From<DynViewMethodBuilder<Vec<Address>>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<Vec<Address>>) -> Self {
    Self::VecOfAddress(builder)
  }
}

impl From<DynViewMethodBuilder<Vec<U256>>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<Vec<U256>>) -> Self {
    Self::VecOfU256(builder)
  }
}

impl From<DynViewMethodBuilder<Void>> for CallReturn {
  fn from(builder: DynViewMethodBuilder<Void>) -> Self {
    Self::Void(builder)
  }
}

impl<T> From<T> for CallReturn
where
  T: Into<SendReturn>,
{
  fn from(builder: T) -> Self {
    match builder.into() {
      SendReturn::Bool(builder) => CallReturn::Bool(builder.view()),
      SendReturn::Void(builder) => CallReturn::Void(builder.view()),
    }
  }
}

pub enum SendReturn {
  Void(DynMethodBuilder<Void>),
  Bool(DynMethodBuilder<bool>),
}

impl From<DynMethodBuilder<Void>> for SendReturn {
  fn from(builder: DynMethodBuilder<Void>) -> Self {
    Self::Void(builder)
  }
}

impl From<DynMethodBuilder<bool>> for SendReturn {
  fn from(builder: DynMethodBuilder<bool>) -> Self {
    Self::Bool(builder)
  }
}
