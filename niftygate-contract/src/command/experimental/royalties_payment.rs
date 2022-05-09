use super::super::SendReturn;
use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  Address, U256,
};
use niftygate_bindings::benber86::nft_royalties_market::royalties_payment::Contract;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  #[structopt(long = "payees", value_name = "H160", use_delimiter = true)]
  payees: Vec<Address>,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(web3, self.payees)
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  Withdraw {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  WithdrawAll,
  PayAll,
  RemovePayee {
    #[structopt(long, value_name = "H160")]
    payee: Address,
  },
  AddPayee {
    #[structopt(long, value_name = "H160")]
    payee: Address,
  },
  #[structopt(name = "WithdrawERC20")]
  WithdrawERC20 {
    #[structopt(long, value_name = "H160")]
    token: Address,
  },
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::AddPayee { payee } => contract.add_payee(payee).into(),
      Self::RemovePayee { payee } => contract.remove_payee(payee).into(),
      Self::PayAll => contract.pay_all().into(),
      Self::Withdraw { amount } => contract.withdraw(amount).into(),
      Self::WithdrawAll => contract.withdraw_all().into(),
      Self::WithdrawERC20 { token } => contract.withdraw_erc_20(token).into()
    }
  }
}
