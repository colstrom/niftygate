use crate::compatibility::Path;
use camino::Utf8Path;
use regex::Regex;
use std::io::{Read, Write};

pub trait Storage {
  type Path: Path;
  type Error: std::error::Error
    + From<std::io::Error>
    + From<<Self::Path as Path>::Error>
    + Send
    + Sync
    + 'static;

  fn root(&self) -> &Self::Path;

  fn reader(&self, path: impl AsRef<Utf8Path>) -> Result<Option<Box<dyn Read>>, Self::Error> {
    Ok(self.root().join(path)?.open_file_if_exists()?)
  }

  fn writer(&self, path: impl AsRef<Utf8Path>) -> Result<Box<dyn Write>, Self::Error> {
    Ok(self.root().join(path)?.create_file_with_parents()?)
  }

  fn read(&self, path: impl AsRef<Utf8Path>) -> Result<Option<Vec<u8>>, Self::Error> {
    match self.reader(path)? {
      None => Ok(None),
      Some(mut reader) => {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(Some(data))
      }
    }
  }

  fn write(&self, path: impl AsRef<Utf8Path>, data: &[u8]) -> Result<(), Self::Error> {
    let mut file = self.writer(path)?;
    file.write_all(data)?;
    file.flush()?;
    Ok(())
  }
}

pub trait StorageExt
where
  Self: Storage,
{
  fn files(&self, pattern: Option<&Regex>) -> Result<Vec<Self::Path>, <Self::Path as Path>::Error> {
    use crate::iter::filter;
    self
      .root()
      .walk_dir()?
      .filter(filter::is_file)
      .filter(filter::is_match_optional(pattern))
      .collect()
  }

  fn files_with_sizes(
    &self,
    pattern: Option<&Regex>,
  ) -> Result<Vec<(Self::Path, u64)>, <Self::Path as Path>::Error> {
    use crate::iter::{filter, map};
    self
      .root()
      .walk_dir()?
      .filter(filter::is_file)
      .filter(filter::is_match_optional(pattern))
      .map(map::with_filesize)
      .collect()
  }
}

impl<T> StorageExt for T where T: Storage {}
