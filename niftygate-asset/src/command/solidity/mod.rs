use semver::{Version, VersionReq};
use structopt::StructOpt;

mod artifact;
mod command;

pub use command::Command;

#[derive(Debug, StructOpt)]
pub struct VersionFilterOptions {
  #[structopt(flatten)]
  target: TargetVersionOption,
  #[structopt(flatten)]
  requirement: VersionRequirementOption,
  #[structopt(long, takes_value = false)]
  all: bool,
}

#[derive(Debug, StructOpt)]
pub struct TargetVersionOption {
  #[structopt(
    short = "T",
    long = "target",
    value_name = "version",
    help = "target a specific version"
  )]
  target: Option<Version>,
}

#[derive(Debug, StructOpt)]
pub struct VersionRequirementOption {
  #[structopt(
    short = "R",
    long = "requirement",
    value_name = "constraint",
    help = "limit to versions meeting requirement"
  )]
  requirement: Option<VersionReq>,
}

use crate::solidity::{BuildMetadata, Manifest};

impl VersionFilterOptions {
  fn filter<'manifest>(&self, manifest: &Manifest) -> Vec<Version> {
    let Self {
      all,
      target: TargetVersionOption { target },
      requirement: VersionRequirementOption { requirement },
    } = self;

    if *all {
      manifest.versions()
    } else {
      match target {
        Some(version) => vec![version],
        None => match requirement {
          None => vec![manifest.latest_release()],
          Some(requirement) => manifest
            .filter_builds_by_requirement(requirement)
            .into_iter()
            .map(BuildMetadata::version)
            .collect(),
        },
      }
    }
    .into_iter()
    .map(Clone::clone)
    .collect()
  }
}
