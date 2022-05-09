// use std::path::{Path, PathBuf};
// pub use vfs::{
//   impls::{altroot::AltrootFS, overlay::OverlayFS},
//   FileSystem, MemoryFS, PhysicalFS, VfsFileType, VfsPath, VfsResult, WalkDirIterator,
// };

// pub trait FileSystemExt
// where
//   Self: FileSystem,
// {
//   fn create_dir_all(&self, path: impl AsRef<Path>) -> VfsResult<()> {
//     let mut tree = PathBuf::new();

//     for path in path.as_ref().components() {
//       tree.push(path);
//       let path = tree.to_string_lossy();
//       if !self.exists(&path)? {
//         self.create_dir(&path)?;
//       }
//     }

//     Ok(())
//   }

//   fn parent(&self, path: impl AsRef<Path>) -> VfsResult<Option<PathBuf>> {
//     let mut parent = PathBuf::new();
//     let mut components = path.as_ref().components().peekable();
//     while let Some(component) = components.next() {
//       if components.peek().is_some() {
//         parent.push(component)
//       }
//     }

//     if PathBuf::new().eq(&parent) {
//       Ok(None)
//     } else {
//       Ok(Some(parent))
//     }
//   }
// }

// impl<T> FileSystemExt for T where T: FileSystem + ?Sized {}
