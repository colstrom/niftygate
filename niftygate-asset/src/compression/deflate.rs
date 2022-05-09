use super::Compression;
use flate2::read::DeflateDecoder as Decoder;
use flate2::write::DeflateEncoder as Encoder;
use flate2::Compression as Level;
use std::io::{Read, Write};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct DeflateCompression {
  level: Level,
}

impl DeflateCompression {
  pub fn none() -> Self {
    Self {
      level: Level::none(),
    }
  }

  pub fn fast() -> Self {
    Self {
      level: Level::fast(),
    }
  }

  pub fn best() -> Self {
    Self {
      level: Level::best(),
    }
  }
}

impl Compression for DeflateCompression {
  fn decoder<'a>(&self, r: Box<dyn Read + 'a>) -> Box<dyn Read + 'a> {
    Box::new(Decoder::new(r))
  }

  fn encoder<'a>(&self, w: Box<dyn Write + 'a>) -> Box<dyn Write + 'a> {
    Box::new(Encoder::new(w, self.level))
  }
}
