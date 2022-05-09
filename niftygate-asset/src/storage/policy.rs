use super::Storage;
use camino::Utf8Path;
use std::io::Read;
use structopt::clap::arg_enum;

pub struct StoragePolicyEnforcer<S>
where
  S: Storage,
{
  pub policy: StoragePolicy,
  pub storage: S,
}

impl<S> StoragePolicyEnforcer<S>
where
  S: Storage,
{
  pub fn new(policy: StoragePolicy, storage: S) -> Self {
    Self { policy, storage }
  }
}

impl<S> Storage for StoragePolicyEnforcer<S>
where
  S: Storage,
{
  type Error = <S as Storage>::Error;
  type Path = <S as Storage>::Path;

  fn root(&self) -> &Self::Path {
    self.storage.root()
  }

  fn reader(&self, path: impl AsRef<Utf8Path>) -> Result<Option<Box<dyn Read>>, Self::Error> {
    if self.policy.permits(StorageOperation::Read) {
      self.storage.reader(path)
    } else {
      Ok(None)
    }
  }

  fn write(&self, path: impl AsRef<Utf8Path>, input: &[u8]) -> Result<(), Self::Error> {
    if self.policy.permits(StorageOperation::Write) {
      self.storage.write(path, input)
    } else {
      Ok(())
    }
  }
}

arg_enum! {
  #[derive(Debug)]
  pub enum StoragePolicy {
    ReadWrite,
    ReadOnly,
    WriteOnly,
    Ignore,
  }
}

impl Default for StoragePolicy {
  fn default() -> Self {
    Self::ReadWrite
  }
}

impl StoragePolicy {
  pub fn permits(&self, operation: StorageOperation) -> bool {
    match operation {
      StorageOperation::Read => self.permits_reading(),
      StorageOperation::Write => self.permits_writing(),
    }
  }

  pub(crate) fn permits_reading(&self) -> bool {
    match self {
      Self::ReadOnly | Self::ReadWrite => true,
      Self::WriteOnly | Self::Ignore => false,
    }
  }

  pub(crate) fn permits_writing(&self) -> bool {
    match self {
      Self::WriteOnly | Self::ReadWrite => true,
      Self::ReadOnly | Self::Ignore => false,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum StorageOperation {
  Read,
  Write,
}
