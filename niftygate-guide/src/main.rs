use anyhow::Result;
use niftygate_guide::Command;
use structopt::StructOpt;

fn main() -> Result<()> {
  Command::from_args().execute()
}
