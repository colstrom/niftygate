use crate::compatibility::Path;
use crate::storage::{Storage, StorageExt};
use crate::ui::{pretty_size, PrettySizeUnits};
use dialoguer::theme::ColorfulTheme;
use indicatif::ProgressIterator;
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Cache Management")]
pub enum Command {
  Path(CachePath),
  Contents(CacheContents),
  Usage(UsageCommand),
  Clean(CacheClean),
}

impl Command {
  pub fn execute<S: Storage>(self, cache: S) -> anyhow::Result<()> {
    match self {
      Self::Path(command) => command.execute(cache),
      Self::Contents(command) => command.execute(cache),
      Self::Usage(command) => command.execute(cache),
      Self::Clean(command) => command.execute(cache),
    }
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prints the cache path")]
pub struct CachePath {}

impl CachePath {
  fn execute<S: Storage>(self, cache: S) -> anyhow::Result<()> {
    let path = cache.root().to_string_lossy();

    println!("{path}");

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prints the cache contents")]
pub struct CacheContents {
  #[structopt(
    long,
    short = "P",
    value_name = "regex",
    help = "Only show matching entries"
  )]
  pattern: Option<Regex>,
}

impl CacheContents {
  fn execute<S: Storage>(self, cache: S) -> anyhow::Result<()> {
    for path in cache.files(self.pattern.as_ref())? {
      let path = path.to_string_lossy();
      println!("{path}")
    }

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deletes entries from the cache")]
pub struct CacheClean {
  #[structopt(
    long,
    short = "P",
    value_name = "regex",
    help = "Only delete matching entries"
  )]
  pattern: Option<Regex>,
  #[structopt(long, short = "i", help = "Interactively select files to delete")]
  interactive: bool,
  #[structopt(long, short = "n", help = "Do not actually delete anything")]
  dry_run: bool,
}

impl CacheClean {
  fn execute<S: Storage>(self, cache: S) -> anyhow::Result<()> {
    let pattern = self.pattern.as_ref();
    let files = cache.files(pattern)?;

    let files = if self.interactive {
      let ui = crate::ui::UserInput::<ColorfulTheme>::new();
      ui.select_multiple_as(Path::to_string_lossy, "Select Files", &files)?
    } else {
      files.iter().collect()
    };

    for path in files.iter().progress() {
      let filename = path.to_string_lossy();

      if self.dry_run {
        println!("(dry-run) removed {filename}");
      } else {
        path.remove_file()?;
        println!("removed {filename}")
      }
    }

    Ok(())
  }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Prints the cache size")]
pub struct UsageCommand {
  #[structopt(long, value_name = "regex", help = "Only show matching entries")]
  pattern: Option<Regex>,
  #[structopt(long, short = "n", help = "Do not print each entry in the cache")]
  no_entries: bool,
  #[structopt(long, short = "s", help = "Print the total cache size")]
  summary: bool,
  #[structopt(long, short = "p", help = "Print sizes in human-friendly format")]
  pretty: bool,
  #[structopt(long, short = "U", value_name = "format", case_insensitive = true, possible_values = &PrettySizeUnits::variants(), default_value, help = "Units for pretty output")]
  pretty_units: PrettySizeUnits,
}

impl UsageCommand {
  fn execute<S: Storage>(self, cache: S) -> anyhow::Result<()> {
    let pattern = self.pattern.as_ref();
    let entries = !self.no_entries;
    let mut usage = 0;

    let files = cache.files_with_sizes(pattern)?;
    for (path, size) in files {
      let path = path.to_string_lossy();
      usage += size;
      if entries {
        if self.pretty {
          let size = pretty_size(size, &self.pretty_units);
          println!("{size} {path}")
        } else {
          println!("{size} {path}")
        }
      }
    }

    if self.summary {
      if self.pretty {
        let usage = pretty_size(usage, &self.pretty_units);
        println!("{usage}")
      } else {
        println!("{usage}")
      }
    }

    Ok(())
  }
}
