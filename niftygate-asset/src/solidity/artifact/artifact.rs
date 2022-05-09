use super::super::{BuildMetadata, MetadataError};
use super::extract::{ExtractedWasm, WasmExtractor};
use super::overview::overview;
use super::wasm::WasmArtifact;
use ressa::Parser;
use semver::Version;
use std::convert::TryFrom;

const CONTENT_TYPE_PREFIX: &str = "data:application/octet-stream;base64,";

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ArtifactError {
  #[error("artifact contains invalid unicde: {0}")]
  InvalidUnicode(#[from] std::string::FromUtf8Error),
  #[error("artifact metadata error: {0}")]
  InvalidMetadata(#[from] MetadataError),
  #[error("failed to parse artifact as valid javascript")]
  InvalidJavaScript,
  #[error("embedded wasm does not have the expected content type ({CONTENT_TYPE_PREFIX})")]
  UnexpectedContentType,
  #[error("failed to decode embedded wasm: {0}")]
  InvalidBase64(#[from] base64::DecodeError),
  #[error("failed to decompress embedded wasm: {0}")]
  InvalidCompression(#[from] lz4_flex::block::DecompressError),
  #[error("unable to locate embedded wasm")]
  WasmBinaryNotFound,
}

impl From<ressa::Error> for ArtifactError {
  fn from(_: ressa::Error) -> Self {
    Self::InvalidJavaScript
  }
}

/// A verifiable, versioned release of the Solidity compiler.
///
/// This is a JavaScript program compiled from C++ sources using Emscripten,
/// and is typically some tens of megabytes in size. The JavaScript is
/// essentially a wrapper around some WebAssembly.
///
/// Installing binary releases for the user is platform specific. Compiling
/// from source requires a working C++ toolchain, and this is a non-trivial
/// requirement. Requiring NodeJS or some other JS runtime involves similar
/// challenges.
///
/// If we can extract the WebAssembly from the artifact, however... then we
/// could embed a WASM runtime of some sort, and simplify runtime
/// dependencies for the end user.
///
/// Providing such a runtime or doing anything with these blobs is outside the
/// scope of this crate, but providing the means to extract them is in scope.
///
/// Under ideal circumstances, these blobs could be embedded in NiftyGate
/// releases, but due to licensing complications, this is not viable.
///
pub struct ReleaseArtifact {
  /// The metadata that corresponds to this release.
  ///
  /// The important details here are the version information, and checksums.
  metadata: BuildMetadata,
  /// The `soljson` program itself.
  program: String,
}

impl ReleaseArtifact {
  pub fn new(metadata: BuildMetadata, data: Vec<u8>) -> Result<Self, ArtifactError> {
    let program = String::from_utf8(data)?;

    metadata.verify(program.as_bytes())?;

    Ok(Self { metadata, program })
  }

  pub fn metadata(&self) -> &BuildMetadata {
    &self.metadata
  }

  pub fn program(&self) -> &str {
    &self.program
  }

  fn find_embedded_wasm(&self) -> Result<Option<ExtractedWasm>, ArtifactError> {
    Ok(WasmExtractor::from(self).extract_wasm(Parser::new(self.as_ref())?))
  }

  pub fn try_into_wasm(self) -> Result<WasmArtifact, ArtifactError> {
    use std::time::Instant;
    let start = Instant::now();
    match self.find_embedded_wasm()? {
      None => Err(ArtifactError::WasmBinaryNotFound),
      Some(extracted) => {
        let finish = Instant::now();
        dbg!(finish - start);
        if extracted.is_compressed() {
          let decoded = base64::decode(extracted.data())?;
          let wasm = super::emscripten::lz4::uncompress(&decoded, extracted.uncompressed_size());
          let version = self.metadata.version;
          Ok(WasmArtifact::new(version, wasm))
        } else {
          match extracted.data().strip_prefix(CONTENT_TYPE_PREFIX) {
            None => Err(ArtifactError::UnexpectedContentType),
            Some(encoded) => {
              let wasm = base64::decode(encoded)?;
              let version = self.metadata.version;
              Ok(WasmArtifact::new(version, wasm))
            }
          }
        }
      }
    }
  }

  pub(crate) fn overview(&self) -> Result<(), ressa::Error> {
    overview(self)
  }
}

impl AsRef<BuildMetadata> for ReleaseArtifact {
  fn as_ref(&self) -> &BuildMetadata {
    self.metadata()
  }
}

impl AsRef<Version> for ReleaseArtifact {
  fn as_ref(&self) -> &Version {
    self.metadata().version()
  }
}

impl AsRef<str> for ReleaseArtifact {
  fn as_ref(&self) -> &str {
    self.program()
  }
}

impl AsRef<[u8]> for ReleaseArtifact {
  fn as_ref(&self) -> &[u8] {
    self.program().as_bytes()
  }
}

impl TryFrom<ReleaseArtifact> for WasmArtifact {
  type Error = ArtifactError;

  fn try_from(artifact: ReleaseArtifact) -> std::result::Result<Self, Self::Error> {
    artifact.try_into_wasm()
  }
}
