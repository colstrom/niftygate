use thiserror::Error;

mod ui;
mod vfs;

#[allow(dead_code)]
mod scratch {
  mod constants {
    // https://ipfs.github.io/public-gateway-checker/
    pub(super) const CLOUDFLARE_ETH_GATEWAY: &str = "https://cloudflare-eth.com";
    pub(super) const CLOUDFLARE_IPFS_GATEWAY: &str = "https://cloudflare-ipfs.com";
    pub(super) const INFURA_IPFS_GATEWAY: &str = "https://infura-ipfs.io";
    pub(super) const PROTOCOL_LABS_IPFS_GATEWAY: &str = "https://gateway.ipfs.io";
  }

  #[non_exhaustive]
  enum IpfsProvider {
    Cloudflare,
    ProtocolLabs,
    Infura,
  }

  impl IpfsProvider {
    fn gateway(&self) -> &str {
      match self {
        Self::Cloudflare => constants::CLOUDFLARE_IPFS_GATEWAY,
        Self::ProtocolLabs => constants::PROTOCOL_LABS_IPFS_GATEWAY,
        Self::Infura => constants::INFURA_IPFS_GATEWAY,
      }
    }
  }

  #[non_exhaustive]
  enum EthProvider {
    Cloudflare,
  }

  impl EthProvider {
    fn gateway(&self) -> &str {
      match self {
        Self::Cloudflare => constants::CLOUDFLARE_ETH_GATEWAY,
      }
    }
  }
}

pub(crate) mod constants {
  pub(crate) const DEFAULT_ORIGIN: &str = "https://binaries.soliditylang.org/emscripten-wasm32";
  pub(crate) const DEFAULT_MANIFEST_PATH: &str = "list.json";
}

#[derive(Debug, Error)]
#[error("Parse Error")]
struct ParseError;

mod prelude {
  pub use camino::{Utf8Path, Utf8PathBuf, Utf8Prefix};
  pub use regex::Regex;
  pub use surf::http;
}

mod asset_path;
mod checksum;
mod command;
pub(crate) mod compatibility;
mod compression;
mod download;
mod iter;
mod manager;
mod pattern;
mod solidity;
mod storage;

pub use asset_path::AssetPath;
pub use command::Command;
pub use pattern::*;
