use super::{TargetVersionOption, VersionRequirementOption};
use crate::asset_path::AssetPath;
use crate::constants::{DEFAULT_MANIFEST_PATH, DEFAULT_ORIGIN};
use crate::download::{Downloader, SurfDownloader};
use crate::manager::{AssetManager, AssetManagerOptions};
use crate::prelude::http::Url;
use crate::storage::Storage;
// use policies::NetworkAccessPolicy;
use structopt::StructOpt;
use strum::VariantNames;

// #[derive(Debug, StructOpt)]
// pub struct Policies {
//   #[structopt(long = "network-policy", value_name = "policy", possible_values = NetworkAccessPolicy::VARIANTS, default_value)]
//   pub network: NetworkAccessPolicy,
// }

#[derive(Debug, StructOpt)]
#[structopt(about = "Manage Solidity Assets")]
pub struct Command {
  #[structopt(long, value_name = "url", default_value = DEFAULT_ORIGIN)]
  origin: Url,
  #[structopt(long, value_name = "path", default_value = DEFAULT_MANIFEST_PATH)]
  manifest_path: AssetPath,
  #[structopt(long)]
  omit_hostnames_in_cache_path: bool,
  // #[structopt(flatten)]
  // policies: Policies,
  #[structopt(subcommand)]
  subcommand: SubCommand,
}

impl Command {
  pub async fn execute<S: Storage>(self, cache: S) -> anyhow::Result<()> {
    let downloader = SurfDownloader::default();

    // use crate::download::policy::*;
    // let mut downloader = DownloadPolicyEnforcer::new(SurfDownloader::default());
    // downloader.add_policy(self.policies.network);

    let manager = AssetManager {
      cache,
      downloader,
      options: AssetManagerOptions {
        origin: self.origin,
        manifest_path: self.manifest_path,
        hostnames_in_cache_path: !self.omit_hostnames_in_cache_path,
      },
    };

    match self.subcommand {
      SubCommand::Manifest(command) => command.execute(manager).await?,
      SubCommand::Download(command) => command.execute(manager).await?,
      SubCommand::DownloadBuilds(command) => command.execute(manager).await?,
      SubCommand::Versions(command) => command.execute(manager).await?,
      SubCommand::Artifact(command) => command.execute(manager).await?,
    }

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
  Manifest(ManifestCommand),
  Versions(VersionsCommand),
  Download(DownloadCommand),
  DownloadBuilds(DownloadBuildsCommand),
  Artifact(super::artifact::Command),
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prints Release Manifest JSON to STDOUT")]
pub struct ManifestCommand {
  #[structopt(long, about = "Pretty JSON (default is compact)")]
  pretty: bool,
}

impl ManifestCommand {
  async fn execute<D: Downloader, S: Storage>(
    self,
    manager: AssetManager<S, D>,
  ) -> anyhow::Result<()> {
    let manifest = manager.manifest().await?;
    let json = manifest.to_json_string(self.pretty)?;

    println!("{json}");

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prints known Solidity Versions")]
pub struct VersionsCommand {
  #[structopt(flatten)]
  version: VersionRequirementOption,
}

impl VersionsCommand {
  async fn execute<D: Downloader, S: Storage>(
    self,
    manager: AssetManager<S, D>,
  ) -> anyhow::Result<()> {
    let manifest = manager.manifest().await?;
    let builds = match self.version.requirement {
      Some(requirement) => manifest.filter_builds_by_requirement(&requirement),
      None => manifest.builds(),
    };

    for build in builds {
      println!("{}", &build.version);
    }

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Downloads Solidity Releases")]
pub struct DownloadCommand {
  #[structopt(flatten)]
  version: TargetVersionOption,
}

impl DownloadCommand {
  async fn execute<D: Downloader, S: Storage>(
    self,
    manager: AssetManager<S, D>,
  ) -> anyhow::Result<()> {
    manager
      .interactive()
      .download_build(self.version.target)
      .await?;
    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Downloads Solidity Builds")]
pub struct DownloadBuildsCommand {
  #[structopt(flatten)]
  version: VersionRequirementOption,
  #[structopt(long, help = "Download all available versions")]
  all: bool,
}

impl DownloadBuildsCommand {
  async fn execute<D: Downloader, S: Storage>(
    self,
    manager: AssetManager<S, D>,
  ) -> anyhow::Result<()> {
    manager
      .interactive()
      .download_builds(self.version.requirement, self.all)
      .await?;
    Ok(())
  }
}
