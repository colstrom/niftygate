use super::FileType;

pub trait Metadata {
  type FileType: FileType;

  fn len(&self) -> u64;
  fn file_type(&self) -> Self::FileType;

  fn is_file(&self) -> bool {
    self.file_type().is_file()
  }

  fn is_dir(&self) -> bool {
    self.file_type().is_dir()
  }
}

impl Metadata for std::fs::Metadata {
  type FileType = std::fs::FileType;

  fn len(&self) -> u64 {
    self.len()
  }

  fn file_type(&self) -> Self::FileType {
    self.file_type()
  }
}

impl Metadata for vfs::VfsMetadata {
  type FileType = vfs::VfsFileType;

  fn len(&self) -> u64 {
    self.len
  }

  fn file_type(&self) -> Self::FileType {
    self.file_type
  }
}
