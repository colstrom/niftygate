use anyhow::Result;
use structopt::StructOpt;

pub mod generate;

#[derive(Debug, StructOpt)]
#[structopt(about = "Utilities for dealing with Certificates")]
pub enum Command {
  Generate(generate::Command),
}

impl Command {
  pub fn execute(self) -> Result<()> {
    match self {
      Self::Generate(command) => command.execute(),
    }
  }
}
