use super::BuildMetadata;
use crate::AssetPath;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
  #[error("manifest parse error: {0}")]
  ParseError(#[from] serde_json::Error),
  #[error("version not found in manifest: {0}")]
  VersionNotFound(Version),
}

/// A solidity release manifest, detailing a set of known builds.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Manifest {
  /// a sorted Vec of available builds (newest first).
  #[serde(deserialize_with = "sorted_builds")]
  builds: Vec<BuildMetadata>,
  /// a HashMap of versions to filenames.
  releases: HashMap<Version, AssetPath>,
  #[serde(rename = "latestRelease")]
  /// the latest version.
  latest_release: Version,
}

fn sorted_builds<'de, D>(deserializer: D) -> Result<Vec<BuildMetadata>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let mut builds: Vec<BuildMetadata> = Vec::deserialize(deserializer)?;
  builds.sort_unstable_by(|a, b| b.long_version.cmp(&a.long_version));
  Ok(builds)
}

impl Manifest {
  pub fn latest_release(&self) -> &Version {
    &self.latest_release
  }

  pub fn builds(&self) -> Vec<&BuildMetadata> {
    self.builds.iter().collect()
  }

  pub fn versions(&self) -> Vec<&Version> {
    self.builds.iter().map(BuildMetadata::version).collect()
  }

  pub fn filter_builds_by_requirement(&self, requirement: &VersionReq) -> Vec<&BuildMetadata> {
    self
      .builds
      .iter()
      .filter(|&metadata| metadata.matches_requirement(requirement))
      .collect()
  }

  pub fn filter_versions_by_requirement(&self, requirement: &VersionReq) -> Vec<&Version> {
    self
      .filter_builds_by_requirement(requirement)
      .into_iter()
      .map(BuildMetadata::version)
      .collect()
  }

  pub fn find_build_by_version(&self, version: &Version) -> Result<&BuildMetadata, ManifestError> {
    self
      .builds
      .iter()
      .find(|&metadata| metadata.version.eq(version))
      .ok_or_else(|| ManifestError::VersionNotFound(version.to_owned()))
  }

  pub fn latest_build(&self) -> Result<&BuildMetadata, ManifestError> {
    self.find_build_by_version(&self.latest_release)
  }

  pub(crate) fn to_json_string(&self, pretty: bool) -> serde_json::Result<String> {
    if pretty {
      serde_json::to_string_pretty(&self)
    } else {
      serde_json::to_string(&self)
    }
  }
}

impl TryFrom<Vec<u8>> for Manifest {
  type Error = ManifestError;

  fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
    Ok(serde_json::from_slice(&raw)?)
  }
}
