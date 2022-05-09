use anyhow::Result;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
  niftygate_contract::Command::from_args().execute().await
}
