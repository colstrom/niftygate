use crate::middleware::ethereum::BalanceScale;
use anyhow::Result;
use ethcontract::U256;
use std::str::FromStr;
use structopt::StructOpt;
use strum::VariantNames;

#[derive(Debug, StructOpt)]
#[structopt(about = "Prints a table of recognized units and scaling values")]
pub enum Command {
  #[structopt(about = "Prints a table of recognized units and scaling values")]
  Show,
  Convert {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    #[structopt(long, value_name = "unit", default_value = "Wei")]
    scale: BalanceScale,
  },
}

impl Command {
  fn show(&self) -> Result<()> {
    if let Some(max) = BalanceScale::VARIANTS.iter().map(|&s| s.len()).max() {
      for &variant in BalanceScale::VARIANTS {
        let scale = BalanceScale::from_str(variant)?.scale();
        println!("{:<pad$} => {}", &variant, scale, pad = max);
      }
    };

    Ok(())
  }

  fn convert(&self, amount: &U256, scale: &BalanceScale) -> Result<()> {
    let scaled = amount * scale.scale();
    println!("{}", scaled);
    Ok(())
  }

  pub fn execute(self) -> Result<()> {
    match self {
      Self::Show => self.show(),
      Self::Convert { amount, scale } => self.convert(&amount, &scale),
    }
  }
}
