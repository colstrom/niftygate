use super::{TargetVersionOption, VersionFilterOptions};
use crate::compression::CompressionAlgorithm;
use crate::download::Downloader;
use crate::manager::AssetManager;
use crate::storage::Storage;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "(EXPERIMENTAL) Solidity Artifact Features")]
pub enum Command {
  #[structopt(about = "Extracts WebAssembly Binary from a Solidity Artifact")]
  ExtractWASM {
    #[structopt(flatten)]
    version: VersionFilterOptions,
  },
  #[structopt(about = "Prints an overview of a Solidity Artifact")]
  Overview {
    #[structopt(flatten)]
    version: VersionFilterOptions,
  },
  #[structopt(about = "Compress a Solidity Artifact")]
  Compress {
    #[structopt(flatten)]
    version: TargetVersionOption,
  },
}

impl Command {
  pub async fn execute<D: Downloader, S: Storage>(
    self,
    manager: AssetManager<S, D>,
  ) -> anyhow::Result<()> {
    match self {
      Self::ExtractWASM { version } => {
        let manifest = manager.manifest().await?;
        let versions = version.filter(&manifest);
        for version in versions {
          let prefix = format!("{}.{}.{}", version.major, version.minor, version.patch);
          let artifact = manager.artifact_from_version(&version).await?;
          match artifact.try_into_wasm() {
            Ok(solidity) => println!("{prefix} WASM is {} bytes", solidity.wasm().len()),
            Err(error) => println!("{prefix} Failed to extract WASM: {}", error),
          }
        }
      }
      Self::Overview { version } => {
        let manifest = manager.manifest().await?;
        let versions = version.filter(&manifest);
        for version in versions {
          let artifact = manager.artifact_from_version(&version).await?;
          artifact.overview().unwrap();
        }
      }
      Self::Compress { version } => {
        let artifact = match version.target {
          Some(version) => manager.artifact_from_version(&version).await?,
          None => manager.latest_artifact().await?,
        };
        CompressionAlgorithm::compare(artifact)?
      }
    }
    Ok(())
  }
}
