use crate::compression::CompressionAlgorithm;
use crate::storage::policy::{StoragePolicy, StoragePolicyEnforcer};
use crate::storage::{CompressedStorage, DirectStorage};
use anyhow::Result;
use platform_path::{Project, ProjectOptions};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Asset Management")]
pub struct Command {
  #[structopt(long, value_name = "policy", case_insensitive = true, possible_values = &StoragePolicy::variants(), default_value)]
  cache_policy: StoragePolicy,
  #[structopt(long, value_name = "algorithm", case_insensitive = true, possible_values = &CompressionAlgorithm::variants(), default_value)]
  compression: CompressionAlgorithm,
  #[structopt(subcommand)]
  subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
  Cache(super::cache::Command),
  Solidity(super::solidity::Command),
}

pub(crate) const PROJECT_QUALIFIER: &str = "com.suse";
pub(crate) const PROJECT_ORGANIZATION: &str = "SUSE Software Solutions";
pub(crate) const PROJECT_APPLICATION: &str = "NiftyGate";

impl Command {
  pub async fn execute(self) -> Result<()> {
    let root = {
      let options = ProjectOptions {
        qualifier: Some(PROJECT_QUALIFIER.to_owned()),
        organization: Some(PROJECT_ORGANIZATION.to_owned()),
        application: PROJECT_APPLICATION.to_owned(),
      };

      Project::Cache.utf8_path_buf(&options)?.into_std_path_buf()
    };

    let cache = StoragePolicyEnforcer {
      policy: self.cache_policy,
      storage: CompressedStorage {
        compression: self.compression,
        storage: DirectStorage { root },
      },
    };

    match self.subcommand {
      SubCommand::Cache(command) => command.execute(cache)?,
      SubCommand::Solidity(command) => command.execute(cache).await?,
    }

    Ok(())
  }
}
