use super::Compression;
use lz4_flex::frame::FrameDecoder as Decoder;
use lz4_flex::frame::FrameEncoder as Encoder;
use std::io::{Read, Write};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Lz4Compression;

impl Compression for Lz4Compression {
  fn decoder<'a>(&self, rdr: Box<dyn Read + 'a>) -> Box<dyn Read + 'a> {
    Box::new(Decoder::new(rdr))
  }

  fn encoder<'a>(&self, wtr: Box<dyn Write + 'a>) -> Box<dyn Write + 'a> {
    Box::new(WrappedEncoder::new(wtr))
  }
}

struct WrappedEncoder<W>(Encoder<W>)
where
  W: Write;

impl<W> WrappedEncoder<W>
where
  W: Write,
{
  pub fn new(wtr: W) -> Self {
    Self(Encoder::new(wtr))
  }
}

impl<W> Write for WrappedEncoder<W>
where
  W: Write,
{
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.0.write(buf)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.0.flush()
  }

  fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
    self.0.write_all(buf)?;
    self.0.try_finish()?;
    Ok(())
  }
}
