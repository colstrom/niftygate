use anyhow::Result;
use niftygate_certificate::command::Command;
use structopt::StructOpt;

fn main() -> Result<()> {
  Command::from_args().execute()
}
