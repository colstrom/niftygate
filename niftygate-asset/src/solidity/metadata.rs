use crate::asset_path::AssetPath;
use crate::checksum::{Checksum, ChecksumError, Keccak256Checksum, Sha256Checksum};
use crate::prelude::http::Url;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
  #[error("verification failed: {0}")]
  VerificationFailed(#[from] ChecksumError),
}

/// The metadata for a given build of the solidity compiler.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct BuildMetadata {
  /// the path for the artifact
  pub path: AssetPath,
  /// the artifact version (SemVer)
  pub version: Version,
  /// additional build information (SemVer)
  pub build: String,
  #[serde(rename = "longVersion")]
  /// the version with build information (SemVer)
  pub long_version: Version,
  /// the keccak256 checksum for the artifact
  pub keccak256: Keccak256Checksum,
  /// the sha256 checksum for the artifact
  pub sha256: Sha256Checksum,
  /// a list of (decentralized) URLs for this artifact
  pub urls: Vec<Url>,
}

impl BuildMetadata {
  pub(crate) fn version(&self) -> &Version {
    &self.long_version
  }

  pub(crate) fn matches_requirement(&self, requirement: &VersionReq) -> bool {
    requirement.matches(&self.long_version)
  }

  pub fn verify(&self, data: impl AsRef<[u8]>) -> Result<(), MetadataError> {
    let data = data.as_ref();

    self.keccak256.verify(data)?;
    self.sha256.verify(data)?;

    Ok(())
  }

  pub fn ipfs_url(&self) -> Option<&Url> {
    self.urls.iter().find(|url| url.scheme().eq("dweb"))
  }

  pub fn swarm_url(&self) -> Option<&Url> {
    self.urls.iter().find(|url| url.scheme().eq("bzzr"))
  }
}

impl AsRef<Version> for BuildMetadata {
  fn as_ref(&self) -> &Version {
    self.version()
  }
}

impl std::fmt::Display for BuildMetadata {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.version().fmt(f)
  }
}
