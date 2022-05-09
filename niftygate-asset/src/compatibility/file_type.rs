pub trait FileType {
  fn is_file(&self) -> bool;
  fn is_dir(&self) -> bool;
}

impl FileType for std::fs::FileType {
  fn is_file(&self) -> bool {
    self.is_file()
  }

  fn is_dir(&self) -> bool {
    self.is_dir()
  }
}

impl FileType for vfs::VfsFileType {
  fn is_file(&self) -> bool {
    matches!(self, Self::File)
  }

  fn is_dir(&self) -> bool {
    matches!(self, Self::Directory)
  }
}
