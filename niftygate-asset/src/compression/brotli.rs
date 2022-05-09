use super::Compression;
use brotli::enc::BrotliEncoderParams as Params;
use brotli::CompressorWriter as Encoder;
use brotli::Decompressor as Decoder;
use std::io::{Read, Write};

#[derive(Clone, Debug, Default)]
pub struct BrotliCompression {
  buffer_size: usize,
  params: Params,
}

impl Compression for BrotliCompression {
  fn decoder<'a>(&self, r: Box<dyn Read + 'a>) -> Box<dyn Read + 'a> {
    Box::new(Decoder::new(r, self.buffer_size))
  }

  fn encoder<'a>(&self, w: Box<dyn Write + 'a>) -> Box<dyn Write + 'a> {
    Box::new(Encoder::with_params(w, self.buffer_size, &self.params))
  }
}
