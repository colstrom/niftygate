use super::{FileType, Metadata};
use camino::Utf8Path;
use regex::Regex;
use std::borrow::Cow;
use std::io::{Read, Write};

pub trait Path
where
  Self: Sized + std::fmt::Debug,
{
  type Error: std::error::Error + Send + Sync + 'static;
  type Metadata: Metadata;
  type Reader: Read;
  type Writer: Write;

  const MAIN_SEPARATOR: char;

  fn join(&self, path: impl AsRef<Utf8Path>) -> Result<Self, Self::Error>;

  fn exists(&self) -> Result<bool, Self::Error>;
  fn open_file(&self) -> Result<Box<dyn Read>, Self::Error>;
  fn open_file_if_exists(&self) -> Result<Option<Box<dyn Read>>, Self::Error> {
    if self.exists()? {
      Ok(Some(self.open_file()?))
    } else {
      Ok(None)
    }
  }

  fn parent(&self) -> Option<Self>;
  fn create_dir_all(&self) -> Result<(), Self::Error>;
  fn create_file(&self) -> Result<Box<dyn Write>, Self::Error>;
  fn create_file_with_parents(&self) -> Result<Box<dyn Write>, Self::Error> {
    self
      .parent()
      .map_or(Ok(()), |parent| parent.create_dir_all())?;
    self.create_file()
  }

  fn remove_dir(&self) -> Result<(), Self::Error>;
  fn remove_file(&self) -> Result<(), Self::Error>;

  fn walk_dir(&self) -> Result<Box<dyn Iterator<Item = Result<Self, Self::Error>>>, Self::Error>;
  fn metadata(&self) -> Result<Self::Metadata, Self::Error>;
  fn with_metadata(self) -> Result<(Self, Self::Metadata), Self::Error> {
    self.metadata().map(|metadata| (self, metadata))
  }
  fn is_dir(&self) -> Result<bool, Self::Error> {
    self
      .metadata()
      .map(|metadata| metadata.file_type().is_dir())
  }
  fn is_file(&self) -> Result<bool, Self::Error> {
    self
      .metadata()
      .map(|metadata| metadata.file_type().is_file())
  }
  fn filesize(&self) -> Result<u64, Self::Error> {
    self.metadata().map(|metadata| metadata.len())
  }
  fn with_filesize(self) -> Result<(Self, u64), Self::Error> {
    self.filesize().map(|filesize| (self, filesize))
  }

  fn to_unicode_string(&self) -> Option<String>;
  fn to_string_lossy(&self) -> Cow<'_, str>;
  fn is_match(&self, pattern: &Regex) -> bool {
    self
      .to_unicode_string()
      .map_or(false, |filename| pattern.is_match(&filename))
  }
  fn is_match_optional(&self, pattern: Option<&Regex>) -> bool {
    pattern.map_or(true, |pattern| self.is_match(pattern))
  }
}

impl Path for std::path::PathBuf {
  type Error = std::io::Error;
  type Metadata = std::fs::Metadata;
  type Reader = std::fs::File;
  type Writer = std::fs::File;

  const MAIN_SEPARATOR: char = std::path::MAIN_SEPARATOR;

  fn join(&self, path: impl AsRef<Utf8Path>) -> Result<Self, Self::Error> {
    Ok(self.as_path().join(path.as_ref()))
  }

  #[rustversion::stable]
  fn exists(&self) -> Result<bool, Self::Error> {
    Ok(self.as_path().exists())
  }

  #[rustversion::nightly]
  fn exists(&self) -> Result<bool, Self::Error> {
    std::fs::try_exists(&self)
  }

  fn open_file(&self) -> Result<Box<dyn Read>, Self::Error> {
    let file = std::fs::File::open(&self)?;
    Ok(Box::new(file))
  }

  fn parent(&self) -> Option<Self> {
    self.as_path().parent().map(|path| path.to_path_buf())
  }

  fn create_dir_all(&self) -> Result<(), Self::Error> {
    std::fs::create_dir_all(&self)
  }

  fn create_file(&self) -> Result<Box<dyn Write>, Self::Error> {
    let file = std::fs::File::create(&self)?;
    Ok(Box::new(file))
  }

  fn remove_dir(&self) -> Result<(), Self::Error> {
    std::fs::remove_dir(&self)
  }

  fn remove_file(&self) -> Result<(), Self::Error> {
    std::fs::remove_file(&self)
  }

  fn metadata(&self) -> Result<Self::Metadata, Self::Error> {
    std::fs::metadata(&self)
  }

  fn to_unicode_string(&self) -> Option<String> {
    self.to_str().map(|filename| filename.to_string())
  }

  fn to_string_lossy(&self) -> Cow<'_, str> {
    self.as_path().to_string_lossy()
  }

  fn walk_dir(&self) -> Result<Box<dyn Iterator<Item = Result<Self, Self::Error>>>, Self::Error> {
    let iter = walkdir::WalkDir::new(&self)
      .into_iter()
      .map(|result| result.map(|ok| ok.into_path()).map_err(|err| err.into()));

    Ok(Box::new(iter))
  }
}

impl Path for vfs::VfsPath {
  type Error = vfs::VfsError;
  type Metadata = vfs::VfsMetadata;
  type Reader = Box<dyn vfs::SeekAndRead>;
  type Writer = Box<dyn Write>;

  const MAIN_SEPARATOR: char = '/';

  fn join(&self, path: impl AsRef<Utf8Path>) -> Result<Self, Self::Error> {
    self.join(path.as_ref().as_str())
  }

  fn exists(&self) -> Result<bool, Self::Error> {
    self.exists()
  }

  fn open_file(&self) -> Result<Box<dyn Read>, Self::Error> {
    Ok(Box::new(self.open_file()?))
  }

  fn parent(&self) -> Option<Self> {
    self.parent()
  }

  fn create_dir_all(&self) -> Result<(), Self::Error> {
    self.create_dir_all()
  }

  fn create_file(&self) -> Result<Box<dyn Write>, Self::Error> {
    self.create_file()
  }

  fn remove_dir(&self) -> Result<(), Self::Error> {
    self.remove_dir()
  }

  fn remove_file(&self) -> Result<(), Self::Error> {
    self.remove_file()
  }

  fn metadata(&self) -> Result<Self::Metadata, Self::Error> {
    self.metadata()
  }

  fn to_unicode_string(&self) -> Option<String> {
    Some(self.filename())
  }

  fn to_string_lossy(&self) -> Cow<'_, str> {
    self.filename().into()
  }

  fn walk_dir(&self) -> Result<Box<dyn Iterator<Item = Result<Self, Self::Error>>>, Self::Error> {
    Ok(Box::new(self.walk_dir()?))
  }
}
