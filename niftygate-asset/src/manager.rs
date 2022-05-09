use super::download::Downloader;
use super::solidity::artifact::{ArtifactError, ReleaseArtifact};
use super::solidity::{BuildMetadata, Manifest, ManifestError};
use super::storage::Storage;
use super::AssetPath;
use crate::prelude::http::Url;
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, ProgressStyle};
use semver::{Version, VersionReq};
use std::convert::TryInto;

#[derive(Debug, thiserror::Error)]
pub enum AssetManagerError {
  #[error("failed to read from cache")]
  CacheReadFailed,
  #[error("failed to write to cache")]
  CacheWriteFailed,
  #[error("failed to download asset")]
  DownloadFailed,
  #[error("manifest error: {0}")]
  ManifestError(#[from] ManifestError),
  #[error("artifact error: {0}")]
  ArtifactError(#[from] ArtifactError),
}

type Result<T> = std::result::Result<T, AssetManagerError>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AssetManagerOptions {
  pub origin: Url,
  pub manifest_path: AssetPath,
  pub hostnames_in_cache_path: bool,
}

/// An asset manager is responsible for bridging between the various components.
#[derive(Debug)]
pub struct AssetManager<S, D>
where
  S: Storage,
  D: Downloader,
{
  pub cache: S,
  pub downloader: D,
  pub options: AssetManagerOptions,
}

impl<S, D> AssetManager<S, D>
where
  S: Storage,
  D: Downloader,
{
  async fn get_raw_asset(&self, path: &AssetPath) -> Result<Vec<u8>> {
    let remote = path.remote(&self.options.origin);
    let local = match self.options.hostnames_in_cache_path {
      true => path.local(Some(&self.options.origin)),
      false => path.local(None),
    };

    match self
      .cache
      .read(&local)
      .map_err(|_| AssetManagerError::CacheReadFailed)?
    {
      Some(cached) => Ok(cached),
      None => self
        .downloader
        .download(&remote)
        .await
        .map_err(|_| AssetManagerError::DownloadFailed)?
        .body_bytes()
        .await
        .map_err(|_| AssetManagerError::DownloadFailed)
        .and_then(|downloaded| {
          self
            .cache
            .write(&local, &downloaded)
            .map_err(|_| AssetManagerError::CacheWriteFailed)
            .map(|_| downloaded)
        }),
    }
  }

  pub async fn manifest(&self) -> Result<Manifest> {
    let manifest = self
      .get_raw_asset(&self.options.manifest_path)
      .await?
      .try_into()?;
    Ok(manifest)
  }

  pub async fn artifact_from_metadata(&self, metadata: &BuildMetadata) -> Result<ReleaseArtifact> {
    let data = self.get_raw_asset(&metadata.path).await?;
    let artifact = ReleaseArtifact::new(metadata.to_owned(), data)?;
    Ok(artifact)
  }

  pub async fn artifact_from_version(&self, version: &Version) -> Result<ReleaseArtifact> {
    let manifest = self.manifest().await?;
    let metadata = manifest.find_build_by_version(version)?;
    self.artifact_from_metadata(metadata).await
  }

  pub async fn latest_artifact(&self) -> Result<ReleaseArtifact> {
    let manifest = self.manifest().await?;
    let metadata = manifest.latest_build()?;
    self.artifact_from_metadata(metadata).await
  }

  pub(crate) fn interactive(&self) -> InteractiveAssetManager<S, D> {
    InteractiveAssetManager { inner: self }
  }
}

#[derive(Debug)]
pub(crate) struct InteractiveAssetManager<'a, S, D>
where
  S: Storage,
  D: Downloader,
{
  inner: &'a AssetManager<S, D>,
}

impl<'a, S, D> InteractiveAssetManager<'a, S, D>
where
  S: Storage,
  D: Downloader,
{
  pub(crate) async fn download_builds(
    &self,
    requirement: Option<VersionReq>,
    all: bool,
  ) -> anyhow::Result<()> {
    let manifest = self.inner.manifest().await?;

    let items = manifest
      .builds()
      .iter()
      .map(|build| {
        let checked = match &requirement {
          Some(requirement) => build.matches_requirement(requirement),
          None => all,
        };
        (build.to_string(), checked)
      })
      .collect::<Vec<(String, bool)>>();

    let theme = dialoguer::theme::ColorfulTheme::default();
    let mut menu = dialoguer::MultiSelect::with_theme(&theme);
    menu.with_prompt("Solidity Compiler Versions");
    menu.items_checked(&items);
    let indices = menu.interact()?;

    let builds = manifest
      .builds()
      .iter()
      .enumerate()
      .filter(|(index, _)| indices.contains(index))
      .map(|(_, &build)| build)
      .collect::<Vec<&BuildMetadata>>();

    let downloads = indicatif::MultiProgress::new();

    let style = indicatif::ProgressStyle::default_spinner()
      .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    let max = builds.len();
    let handles: Vec<_> = builds
      .iter()
      .enumerate()
      .map(|(index, build)| {
        let download = downloads.add(indicatif::ProgressBar::new_spinner());
        download.set_style(style.clone());
        download.set_prefix(format!("[{}/{}]", index, max));
        download.set_message(format!("downloading: {}", build.to_string()));
        std::thread::spawn(move || {
          for _ in 0..3000 {
            download.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(1));
          }
          download.finish_with_message("done")
        })
      })
      .collect();

    for handle in handles {
      let _ = handle.join();
    }

    downloads.join_and_clear()?;

    Ok(())
  }

  pub(crate) async fn download_build(&self, version: Option<Version>) -> anyhow::Result<()> {
    let manifest = self.inner.manifest().await?;
    let metadata = match version {
      Some(version) => manifest.find_build_by_version(&version)?,
      None => {
        let ui = crate::ui::UserInput::<ColorfulTheme>::new();
        ui.select_one(
          "Solidity Compiler Version",
          &manifest.builds(),
          manifest.latest_build().ok().as_ref(),
        )?
      }
    };

    let len = 20 * 1024 * 1024; // assume 20MB as a rough estimate
    let style = ProgressStyle::default_spinner();
    let spinner = ProgressBar::new(len).with_style(style);
    let msg = format!("Downloading Solidity Compiler Version {metadata}");
    spinner.set_message(msg);
    spinner.enable_steady_tick(50);

    match self.inner.artifact_from_metadata(metadata).await {
      Ok(_bytes) => {
        spinner.finish_using_style();
      }
      Err(error) => {
        let msg = format!("failed with error: {}", error);
        spinner.abandon_with_message(msg);
        return Err(error.into());
      }
    }

    Ok(())
  }
}
