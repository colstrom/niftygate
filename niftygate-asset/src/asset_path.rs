use crate::prelude::http::Url;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The path to some asset.
///
/// This is mainly used for mapping remote paths (for downloading) to
/// local paths (for caching).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Deserialize, Serialize)]
pub struct AssetPath(Utf8PathBuf);

impl AssetPath {
  /// returns the relative local path to this asset.
  ///
  /// if an origin is provided, it will be prefixed to the path.
  ///
  /// This allows assets to cached separately by origin (if provided),
  /// or shared (if omitted).
  pub fn local(&self, origin: Option<&Url>) -> Utf8PathBuf {
    let mut components = Vec::new();

    if let Some(origin) = origin {
      if let Some(host) = origin.host_str() {
        components.push(host);
      }

      let prefix = origin.path().trim_matches('/');
      if !prefix.is_empty() {
        components.push(prefix);
      }
    }

    components.push(self.0.as_str());
    let mut sep = [0u8; 4];
    let sep = std::path::MAIN_SEPARATOR.encode_utf8(&mut sep);

    let path = components.join(sep);

    Utf8PathBuf::from(path)
  }

  /// returns the absolute remote url for this asset.
  ///
  /// this allows paths to be handled relative to any path component in
  /// the origin URL.
  pub fn remote(&self, origin: &Url) -> Url {
    let prefix = origin.path().trim_end_matches('/');
    let path = [prefix, self.0.as_str()].join("/");
    let mut url = origin.clone();
    url.set_path(&path);
    url
  }
}

impl AsRef<str> for AssetPath {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl FromStr for AssetPath {
  type Err = <Utf8PathBuf as FromStr>::Err;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Utf8PathBuf::from_str(s).map(Self)
  }
}
