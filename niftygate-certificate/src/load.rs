use crate::RawCertificate;
use anyhow::Result;
use rcgen::KeyPair;
use std::path::Path;

enum Source {
  Unknown,
  PEMFile,
  DERFile,
}

impl<P: AsRef<Path>> From<P> for Source {
  fn from(path: P) -> Self {
    if let Some(extension) = path.as_ref().extension() {
      if let Some(extension) = extension.to_str() {
        return match extension {
          "der" => Self::DERFile,
          "pem" => Self::PEMFile,
          _ => Self::Unknown,
        };
      }
    }
    Self::Unknown
  }
}

pub(crate) fn key_pair(path: &str) -> Result<KeyPair> {
  let key_pair = match Source::from(&path) {
    Source::DERFile => {
      let der = std::fs::read(path)?;
      KeyPair::from_der(&der)?
    }
    Source::PEMFile | Source::Unknown => {
      let pem_str = std::fs::read_to_string(&path)?;
      KeyPair::from_pem(&pem_str)?
    }
  };

  Ok(key_pair)
}

pub(crate) fn certificate(path: &str) -> Result<RawCertificate> {
  let bytes = match Source::from(&path) {
    Source::DERFile => {
      let bytes = std::fs::read(&path)?;
      let (_, _) = x509_parser::parse_x509_certificate(&bytes)?;
      bytes
    }
    Source::PEMFile | Source::Unknown => {
      let input = std::fs::read_to_string(&path)?;
      let bytes = pem::parse(input)?.contents;
      let (_, _) = x509_parser::parse_x509_certificate(&bytes)?;
      bytes
    }
  };

  Ok(RawCertificate(bytes))
}
