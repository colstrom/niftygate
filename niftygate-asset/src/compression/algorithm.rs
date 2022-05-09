use super::*;
use structopt::clap::arg_enum;

arg_enum! {
  #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
  pub enum CompressionAlgorithm {
    Brotli,
    Deflate,
    Lz4,
    Uncompressed,
  }
}

impl Default for CompressionAlgorithm {
  fn default() -> Self {
    Self::Uncompressed
  }
}

impl Compression for CompressionAlgorithm {
  fn decoder<'a>(&self, reader: Box<dyn std::io::Read + 'a>) -> Box<dyn std::io::Read + 'a> {
    match self {
      Self::Uncompressed => NoCompression::default().decoder(reader),
      Self::Deflate => DeflateCompression::default().decoder(reader),
      Self::Lz4 => Lz4Compression::default().decoder(reader),
      Self::Brotli => BrotliCompression::default().decoder(reader),
    }
  }

  fn encoder<'a>(&self, writer: Box<dyn std::io::Write + 'a>) -> Box<dyn std::io::Write + 'a> {
    match self {
      Self::Uncompressed => NoCompression::default().encoder(writer),
      Self::Deflate => DeflateCompression::default().encoder(writer),
      Self::Lz4 => Lz4Compression::default().encoder(writer),
      Self::Brotli => BrotliCompression::default().encoder(writer),
    }
  }
}

impl CompressionAlgorithm {
  pub fn engine(&self) -> Box<dyn Compression> {
    match self {
      Self::Uncompressed => Box::new(NoCompression::default()),
      Self::Lz4 => Box::new(Lz4Compression::default()),
      Self::Deflate => Box::new(DeflateCompression::default()),
      Self::Brotli => Box::new(BrotliCompression::default()),
    }
  }

  pub(crate) fn compare<I>(input: I) -> anyhow::Result<()>
  where
    I: AsRef<[u8]>,
  {
    use sha3::{Digest, Keccak256};
    use std::time::Instant;
    let input = input.as_ref();

    let input_hash = Keccak256::digest(&input);
    let input_size = input.len();
    dbg!(&input_hash, input_size);

    let algorithms = vec![Self::Uncompressed, Self::Lz4, Self::Deflate, Self::Brotli];

    for algorithm in algorithms {
      let engine = algorithm.engine();

      let compressed = Vec::<u8>::with_capacity(input_size);
      let start = Instant::now();
      let compressed = engine.encode_into(compressed, input)?;
      let compress_time = Instant::now() - start;
      let compressed_size = compressed.len();

      let decompressed = Vec::<u8>::with_capacity(input_size);
      let start = Instant::now();
      let decompressed = engine.decode_into(decompressed, &compressed)?;
      let decompress_time = Instant::now() - start;
      let decompressed_size = decompressed.len();

      let output_hash = Keccak256::digest(&decompressed);
      let corrupted = decompressed_size.ne(&input_size) || input_hash.ne(&output_hash);

      let ratio = compressed_size as f32 / decompressed_size as f32;

      dbg!(
        algorithm,
        compress_time,
        decompress_time,
        compressed_size,
        ratio,
        output_hash,
        corrupted
      );
    }

    Ok(())
  }
}
