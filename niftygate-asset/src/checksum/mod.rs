mod algorithm;
mod keccak256;
mod policy;
mod sha256;

pub use algorithm::*;
pub use keccak256::*;
pub use policy::*;
pub use sha256::*;

use digest::Digest;

#[derive(Debug, thiserror::Error)]
pub enum ChecksumError {
  #[error("checksum mismatch (expected {expected:?}, found {found:?})")]
  Mismatch { expected: String, found: String },
}
pub(crate) trait Checksum
where
  Self: AsRef<[u8]> + std::fmt::Display,
{
  type Digest: Digest;

  fn verify(&self, data: impl AsRef<[u8]>) -> Result<(), ChecksumError> {
    let digest = <Self::Digest as Digest>::digest(data).to_vec();
    let expected = self.as_ref();

    if digest.eq(expected) {
      Ok(())
    } else {
      Err(ChecksumError::Mismatch {
        expected: hex::encode(&expected),
        found: hex::encode(&digest),
      })
    }
  }
}
