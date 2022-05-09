#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum ChecksumAlgorithm {
  Keccak256,
  Sha256,
  Unverified,
  All,
}

impl Default for ChecksumAlgorithm {
  fn default() -> Self {
    Self::Keccak256
  }
}
