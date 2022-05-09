use std::io::{self, Read, Write};

pub trait Compression {
  fn decoder<'a>(&self, reader: Box<dyn Read + 'a>) -> Box<dyn Read + 'a>;
  fn encoder<'a>(&self, writer: Box<dyn Write + 'a>) -> Box<dyn Write + 'a>;

  fn decode_into(&self, mut buf: Vec<u8>, encoded: &[u8]) -> io::Result<Vec<u8>> {
    {
      let mut decoder = self.decoder(Box::new(encoded));
      decoder.read_to_end(&mut buf)?;
    }
    Ok(buf)
  }

  fn decode(&self, encoded: &[u8]) -> io::Result<Vec<u8>> {
    self.decode_into(Vec::new(), encoded)
  }

  fn encode_into(&self, mut buf: Vec<u8>, data: &[u8]) -> io::Result<Vec<u8>> {
    {
      let mut encoder = self.encoder(Box::new(&mut buf));
      encoder.write_all(data)?;
      encoder.flush()?;
    }
    Ok(buf)
  }

  fn encode(&self, data: &[u8]) -> io::Result<Vec<u8>> {
    self.encode_into(Vec::new(), data)
  }
}
