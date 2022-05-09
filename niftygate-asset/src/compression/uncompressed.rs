use super::Compression;
use std::io::{Read, Write};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NoCompression;

impl Compression for NoCompression {
  fn decoder<'a>(&self, reader: Box<dyn Read + 'a>) -> Box<dyn Read + 'a> {
    reader
  }

  fn encoder<'a>(&self, writer: Box<dyn Write + 'a>) -> Box<dyn Write + 'a> {
    writer
  }
}
