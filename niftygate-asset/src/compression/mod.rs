mod algorithm;
mod brotli;
#[allow(clippy::module_inception)]
mod compression;
mod deflate;
mod lz4;
mod uncompressed;

pub use self::brotli::*;
pub use algorithm::*;
pub use compression::*;
pub use deflate::*;
pub use lz4::*;
pub use uncompressed::*;
