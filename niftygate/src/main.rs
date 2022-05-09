use anyhow::Result;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
  niftygate::Command::from_args().execute().await
}
