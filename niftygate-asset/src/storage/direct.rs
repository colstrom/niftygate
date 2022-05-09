use super::Storage;
use std::path::PathBuf;

pub struct DirectStorage {
  pub root: PathBuf,
}

impl DirectStorage {
  pub fn new(root: PathBuf) -> Self {
    Self { root }
  }
}

impl Storage for DirectStorage {
  type Error = std::io::Error;
  type Path = std::path::PathBuf;

  fn root(&self) -> &Self::Path {
    &self.root
  }
}
