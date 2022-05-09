use structopt::StructOpt;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  let command = niftygate_asset::Command::from_args();
  command.execute().await?;

  Ok(())
}
