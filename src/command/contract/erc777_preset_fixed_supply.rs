use super::{CallReturn, SendReturn};
use crate::{
  command::HexData,
  openzeppelin::contracts::token::erc777::presets::erc777_preset_fixed_supply::Contract,
};
use ethcontract::{
  dyns::{DynDeployBuilder, DynWeb3},
  Address, U256,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub struct DeployCommand {
  #[structopt(long, value_name = "String")]
  name: String,
  #[structopt(long, value_name = "String")]
  symbol: String,
  #[structopt(long = "defaultOperators", value_name = "H160", use_delimiter = true)]
  default_operators: Vec<Address>,
  #[structopt(long = "initialSupply", value_name = "U256", parse(try_from_str = U256::from_dec_str))]
  initial_supply: U256,
  #[structopt(long, value_name = "H160")]
  owner: Address,
}

impl DeployCommand {
  pub fn build(self, web3: &DynWeb3) -> DynDeployBuilder<Contract> {
    Contract::builder(
      web3,
      self.name,
      self.symbol,
      self.default_operators,
      self.initial_supply,
      self.owner,
    )
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum CallCommand {
  #[structopt(
    about = "Returns the allowance of one account on behalf of another.",
    long_about = "Returns the remaining number of tokens that <spender> will be allowed to spend on behalf of <owner> through {transferFrom}. This is zero by default."
  )]
  Allowance {
    #[structopt(long, value_name = "H160")]
    holder: Address,
    #[structopt(long, value_name = "H160")]
    spender: Address,
  },
  #[structopt(about = "Returns the amount of tokens owned by an account.")]
  BalanceOf {
    #[structopt(long, value_name = "H160")]
    token_holder: Address,
  },
  #[structopt(about = "Returns the number of decimals used to get its user representation.")]
  Decimals,
  #[structopt(
    about = "Returns the list of default operators",
    long_about = "Returns the list of default operators. These accounts are operators for all token holders, even if {authorizeOperator} was never called on them."
  )]
  DefaultOperators,
  #[structopt(
    about = "Returns the smallest part of the token that is not divisible.",
    long_about = "Returns the smallest part of the token that is not divisible. This means all token operations (creation, movement and destruction) must have amounts that are a multiple of this number."
  )]
  Granularity,
  #[structopt(
    about = "Returns true if an account is an operator.",
    long_about = "Returns true if an account is an operator of <tokenHolder>. Operators can send and burn tokens on behalf of their owners. All accounts are their own operator."
  )]
  IsOperatorFor {
    #[structopt(long, value_name = "H160")]
    operator: Address,
    #[structopt(long, value_name = "H160")]
    token_holder: Address,
  },
  #[structopt(about = "Returns the name of the token.")]
  Name,
  #[structopt(
    about = "Returns the symbol of the token.",
    long_about = "Returns the symbol of the token, usually a shorter version of the name."
  )]
  Symbol,
  #[structopt(about = "Returns the amount of tokens in existence.")]
  TotalSupply,
}

impl CallCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> CallReturn {
    let contract = Contract::at(web3, address);
    match self {
      Self::Allowance { holder, spender }
        => contract.allowance(holder, spender).view().into(),
      Self::BalanceOf { token_holder }
        => contract.balance_of(token_holder).view().into(),
      Self::Decimals
        => contract.decimals().view().into(),
      Self::DefaultOperators
        => contract.default_operators().view().into(),
      Self::Granularity
        => contract.granularity().view().into(),
      Self::IsOperatorFor { operator, token_holder }
        => contract.is_operator_for(operator, token_holder).view().into(),
      Self::Name
        => contract.name().view().into(),
      Self::Symbol
        => contract.symbol().view().into(),
      Self::TotalSupply
        => contract.total_supply().view().into(),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "camel")]
pub enum SendCommand {
  #[structopt(
    about = "Sets the allowance over the caller's tokens.",
    long_about = "Sets <value> as the allowance of <spender> over the caller's tokens."
  )]
  Approve {
    #[structopt(long, value_name = "H160")]
    spender: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    value: U256,
  },
  #[structopt(about = "Make an account an operator of the caller.")]
  AuthorizeOperator {
    #[structopt(long, value_name = "H160")]
    operator: Address,
  },
  #[structopt(
    about = "Destroys tokens from the caller's account, reducing the total supply.",
    long_about = "Destroys <amount> tokens from the caller's account, reducing the total supply. If a send hook is registered for the caller, the corresponding function will be called with <data> and empty <operatorData>."
  )]
  Burn {
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
  },
  #[structopt(
    about = "Destroys tokens from an account, reducing the total supply.",
    long_about = "Destroys <amount> tokens from <account>, reducing the total supply. The caller must be an operator of <account>. If a send hook is registered for <account>, the corresponding function will be called with <data> and <operatorData>."
  )]
  OperatorBurn {
    #[structopt(long, value_name = "H160")]
    account: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
    #[structopt(long, value_name = "HexString")]
    operator_data: HexData,
  },
  #[structopt(
    about = "Moves tokens from one acount to another, calling registered hooks.",
    long_about = "Moves <amount> tokens from <sender> to <recipient>. The caller must be an operator of <sender>. If send or receive hooks are registered for <sender> and <recipient>, the corresponding functions will be called with <data> and <operatorData>."
  )]
  OperatorSend {
    #[structopt(long, value_name = "H160")]
    sender: Address,
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    data: HexData,
    #[structopt(long, value_name = "HexString")]
    operator_data: HexData,
  },
  #[structopt(about = "Revoke an account's operator status for the caller.")]
  RevokeOperator {
    #[structopt(long, value_name = "H160")]
    operator: Address,
  },
  #[structopt(
    about = "Moves tokens from the caller's account, calling registered hooks.",
    long_about = "Moves <amount> tokens from the caller's account to <recipient>. If send or receive hooks are registered for the caller and <recipient>, the corresponding functions will be called with <data> and empty <operatorData>."
  )]
  Send {
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
    #[structopt(long, value_name = "HexString")]
    data: HexData,
  },
  #[structopt(
    about = "Moves tokens from the caller's account",
    long_about = "Moves <amount> tokens from the caller's account to <recipient>."
  )]
  Transfer {
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
  #[structopt(
    about = "Moves tokens using the allowance mechanism.",
    long_about = "Moves <amount> tokens from <holder> to <recipient> using the allowance mechanism. <amount> is then deducted from the caller's allowance."
  )]
  TransferFrom {
    #[structopt(long, value_name = "H160")]
    holder: Address,
    #[structopt(long, value_name = "H160")]
    recipient: Address,
    #[structopt(long, value_name = "U256", parse(try_from_str = U256::from_dec_str))]
    amount: U256,
  },
}

impl SendCommand {
  #[rustfmt::skip]
  pub fn build(self, web3: &DynWeb3, address: Address) -> SendReturn {
    let contract = Contract::at(&web3, address);
    match self {
      Self::Approve { spender, value }
        => contract.approve(spender, value).into(),
      Self::AuthorizeOperator { operator }
        => contract.authorize_operator(operator).into(),
      Self::Burn { amount, data }
        => contract.burn(amount, data.0).into(),
      Self::OperatorBurn { account, amount, data, operator_data }
        => contract.operator_burn(account, amount, data.0, operator_data.0).into(),
      Self::OperatorSend { sender, recipient, amount, data, operator_data }
        => contract.operator_send(sender, recipient, amount, data.0, operator_data.0).into(),
      Self::RevokeOperator { operator }
        => contract.revoke_operator(operator).into(),
      Self::Send { recipient, amount, data }
        => contract.send(recipient, amount, data.0).into(),
      Self::Transfer { recipient, amount }
        => contract.transfer(recipient, amount).into(),
      Self::TransferFrom { holder, recipient, amount }
        => contract.transfer_from(holder, recipient, amount).into(),
    }
  }
}
