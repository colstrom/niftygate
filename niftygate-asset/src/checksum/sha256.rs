use super::Checksum;
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictPfx};

#[derive(
  Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Sha256Checksum(#[serde(with = "SerHex::<StrictPfx>")] [u8; 32]);

impl AsRef<[u8]> for Sha256Checksum {
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}

impl std::fmt::Display for Sha256Checksum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", hex::encode(&self.0))
  }
}

impl Checksum for Sha256Checksum {
  type Digest = sha2::Sha256;
}
