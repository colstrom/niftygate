use anyhow::Result;
use structopt::StructOpt;

mod demo;
mod units;
mod web3;

#[derive(Debug, StructOpt)]
pub enum Command {
  Demo(demo::Command),
  Units(units::Command),
  Web3(web3::Command),
  Contract(niftygate_contract::Command),
  Guide(niftygate_guide::Command),
  Certificate(niftygate_certificate::Command),
  Asset(niftygate_asset::Command),
}

impl Command {
  pub async fn execute(self) -> Result<()> {
    match self {
      Self::Demo(command) => command.execute().await?,
      Self::Units(command) => command.execute()?,
      Self::Contract(command) => command.execute().await?,
      Self::Guide(command) => command.execute()?,
      Self::Certificate(command) => command.execute()?,
      Self::Web3(command) => command.execute().await?,
      Self::Asset(command) => command.execute().await?,
    }

    Ok(())
  }
}

#[deprecated(since = "0.7.1", note = "use Command.execute() instead")]
pub async fn run() -> Result<()> {
  Command::from_args().execute().await
}

#[deprecated(
  since = "0.7.1",
  note = "this import path is deprecated, HexData is now in the crate root"
)]
pub use crate::HexData;
