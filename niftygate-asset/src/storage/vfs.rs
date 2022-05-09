use super::Storage;
use camino::Utf8PathBuf;
use vfs::impls::{altroot::AltrootFS, overlay::OverlayFS};
use vfs::{MemoryFS, PhysicalFS, VfsPath};

pub struct VfsStorage {
  root: VfsPath,
}

impl VfsStorage {
  pub fn new(root: VfsPath) -> Self {
    Self { root }
  }
}

impl VfsStorage {
  pub fn physical(root: Utf8PathBuf) -> Self {
    Self::new(VfsPath::new(PhysicalFS::new(root.into())))
  }

  pub fn altroot(root: VfsPath) -> Self {
    Self::new(VfsPath::new(AltrootFS::new(root)))
  }

  pub fn overlay(layers: &[VfsPath]) -> Self {
    Self::new(VfsPath::new(OverlayFS::new(layers)))
  }

  pub fn memory() -> Self {
    Self::new(VfsPath::new(MemoryFS::new()))
  }
}

impl Storage for VfsStorage {
  type Error = vfs::VfsError;
  type Path = vfs::VfsPath;

  fn root(&self) -> &Self::Path {
    &self.root
  }
}
