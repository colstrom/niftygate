use crate::{
  middleware::{
    ethereum::{prelude::*, *},
    *,
  },
  WrappedResult,
};
use tide::Server;

pub struct ERC1155Config {
  pub contract_address: Address,
  pub balance_header: HeaderName,
  pub balance_requirement: Option<BalanceRequirement>,
  pub provides_balances: bool,
}

pub struct ERC20Config {
  pub contract_address: Address,
  pub balance_header: HeaderName,
  pub balance_requirement: Option<BalanceRequirement>,
  pub name_header: HeaderName,
  pub symbol_header: HeaderName,
  pub provides_balances: bool,
  pub provides_name: bool,
  pub provides_symbol: bool,
}

pub struct ERC721Config {
  pub contract_address: Address,
  pub balance_header: HeaderName,
  pub balance_requirement: Option<BalanceRequirement>,
  pub name_header: HeaderName,
  pub symbol_header: HeaderName,
  pub provides_balances: bool,
  pub provides_name: bool,
  pub provides_symbol: bool,
}

pub struct ERC777Config {
  pub contract_address: Address,
  pub balance_header: HeaderName,
  pub balance_requirement: Option<BalanceRequirement>,
  pub name_header: HeaderName,
  pub symbol_header: HeaderName,
  pub provides_balances: bool,
  pub provides_name: bool,
  pub provides_symbol: bool,
}

pub struct Config {
  pub address_header: HeaderName,
  pub backend: Url,
  pub balance_header: HeaderName,
  pub balance_requirement: Option<BalanceRequirement>,
  pub balance_scale: Option<BalanceScale>,
  pub challenge: Vec<u8>,
  pub provides_account_verification: bool,
  pub provides_balances: bool,
  pub erc20: ERC20Config,
  pub erc1155: ERC1155Config,
  pub erc721: ERC721Config,
  pub erc777: ERC777Config,
  pub provides_signatures: bool,
  pub web3_rpc_url: Url,
  pub secret_key: Option<SecretKey>,
  pub signature_header: HeaderName,
}

pub async fn server(config: Config) -> WrappedResult<Server<()>> {
  let mut server = tide::new();
  server.with(ProvidesForwardedHeader);

  let web3 = crate::util::web3_from_url(config.web3_rpc_url).await?;

  if config.provides_signatures {
    server.with(ProvidesSignature {
      signature_header: config.signature_header.clone(),
      secret_key: config
        .secret_key
        .expect("Cannot provide signatures without a secret key!"),
      web3: web3.clone(),
      challenge: config.challenge.clone(),
    });
  }

  if config.provides_account_verification {
    server.with(ProvidesAccountVerification {
      signature_header: config.signature_header.clone(),
      address_header: config.address_header.clone(),
      status_code: StatusCode::PaymentRequired,
      web3: web3.clone(),
      challenge: config.challenge.clone(),
    });
  }

  if config.provides_balances {
    server.with(ProvidesBalance {
      address_header: config.address_header.clone(),
      balance_header: config.balance_header.clone(),
      web3: web3.clone(),
    });
  }

  if let Some(requirement) = config.balance_requirement {
    server.with(
      RequiresBalance {
        header: config.balance_header.clone(),
        requirement,
      }
      .scale(config.balance_scale.unwrap_or(BalanceScale::Gwei)),
    );
  }

  if !config.erc1155.contract_address.is_zero() {
    if config.erc1155.provides_balances {
      server.with(ProvidesERC1155Balance {
        address_header: config.address_header.clone(),
        balance_header: config.erc1155.balance_header.clone(),
        contract: ERC1155::at(&web3, config.erc1155.contract_address),
      });
    }

    if let Some(requirement) = config.erc1155.balance_requirement {
      server.with(RequiresBalance {
        header: config.erc1155.balance_header.clone(),
        requirement,
      });
    }
  }

  if !config.erc20.contract_address.is_zero() {
    if config.erc20.provides_balances {
      let name_header = if config.erc20.provides_name {
        Some(config.erc20.name_header)
      } else {
        None
      };

      let symbol_header = if config.erc20.provides_symbol {
        Some(config.erc20.symbol_header)
      } else {
        None
      };

      server.with(ProvidesERC20Balance {
        address_header: config.address_header.clone(),
        balance_header: config.erc20.balance_header.clone(),
        name_header,
        symbol_header,
        contract: ERC20::at(&web3, config.erc20.contract_address),
      });
    }

    if let Some(requirement) = config.erc20.balance_requirement {
      server.with(RequiresBalance {
        header: config.erc20.balance_header.clone(),
        requirement,
      });
    }
  }

  if !config.erc721.contract_address.is_zero() {
    if config.erc721.provides_balances {
      let name_header = if config.erc721.provides_name {
        Some(config.erc721.name_header)
      } else {
        None
      };

      let symbol_header = if config.erc721.provides_symbol {
        Some(config.erc721.symbol_header)
      } else {
        None
      };

      server.with(ProvidesERC721Balance {
        address_header: config.address_header.clone(),
        balance_header: config.erc721.balance_header.clone(),
        name_header,
        symbol_header,
        contract: ERC721::at(&web3, config.erc721.contract_address),
      });
    }

    if let Some(requirement) = config.erc721.balance_requirement {
      server.with(RequiresBalance {
        header: config.erc721.balance_header.clone(),
        requirement,
      });
    }
  }

  if !config.erc777.contract_address.is_zero() {
    if config.erc777.provides_balances {
      let name_header = if config.erc777.provides_name {
        Some(config.erc777.name_header)
      } else {
        None
      };

      let symbol_header = if config.erc777.provides_symbol {
        Some(config.erc777.symbol_header)
      } else {
        None
      };

      server.with(ProvidesERC777Balance {
        address_header: config.address_header.clone(),
        balance_header: config.erc777.balance_header.clone(),
        name_header,
        symbol_header,
        contract: ERC777::at(&web3, config.erc777.contract_address),
      });
    }

    if let Some(requirement) = config.erc777.balance_requirement {
      server.with(RequiresBalance {
        header: config.erc777.balance_header.clone(),
        requirement,
      });
    }
  }

  server.with(Proxy::new(config.backend));

  Ok(server)
}
