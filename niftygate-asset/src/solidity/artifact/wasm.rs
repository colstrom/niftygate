use super::ReleaseArtifact;
use semver::Version;
use std::convert::TryFrom;

/// A versioned WebAssembly artifact, extracted from a release of soljson.
///
/// Someone could probably write a crate that loads this into an embedded
/// WASM runtime, which would remove the dependency on NodeJS at runtime.
///
/// The version information is included here, even though it is likely
/// contained in the WASM itself, because it may be required in order to
/// load the WASM in the first place.
///
pub struct WasmArtifact {
  version: Version,
  wasm: Vec<u8>,
}

impl WasmArtifact {
  pub fn new(version: Version, wasm: Vec<u8>) -> Self {
    Self { version, wasm }
  }

  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn wasm(&self) -> &[u8] {
    &self.wasm
  }

  pub fn try_from_release(
    artifact: ReleaseArtifact,
  ) -> Result<Self, <Self as TryFrom<ReleaseArtifact>>::Error> {
    Self::try_from(artifact)
  }
}

impl AsRef<Version> for WasmArtifact {
  fn as_ref(&self) -> &Version {
    self.version()
  }
}

impl AsRef<[u8]> for WasmArtifact {
  fn as_ref(&self) -> &[u8] {
    self.wasm()
  }
}
