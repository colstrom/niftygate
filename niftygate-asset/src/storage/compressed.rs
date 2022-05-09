use super::Storage;
use crate::compression::Compression;
use camino::Utf8Path;
use std::io::{Read, Write};

pub struct CompressedStorage<C, S>
where
  C: Compression,
  S: Storage,
{
  pub compression: C,
  pub storage: S,
}

impl<C, S> CompressedStorage<C, S>
where
  C: Compression,
  S: Storage,
{
  pub fn new(compression: C, storage: S) -> Self {
    Self {
      compression,
      storage,
    }
  }
}

impl<C, S> Storage for CompressedStorage<C, S>
where
  C: Compression,
  S: Storage,
{
  type Path = <S as Storage>::Path;
  type Error = <S as Storage>::Error;

  fn root(&self) -> &Self::Path {
    self.storage.root()
  }

  fn reader(&self, path: impl AsRef<Utf8Path>) -> Result<Option<Box<dyn Read>>, Self::Error> {
    self
      .storage
      .reader(path)
      .map(|input| input.map(|input| self.compression.decoder(input)))
  }

  fn writer(&self, path: impl AsRef<Utf8Path>) -> Result<Box<dyn Write>, Self::Error> {
    self
      .storage
      .writer(path)
      .map(|output| self.compression.encoder(output))
  }
}
