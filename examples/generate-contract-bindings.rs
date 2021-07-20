// This is not an actual example, don't use this for reference.
//
// Instead, this program is responsible for (re-)generating the bindings
// found in src/openzeppelin/contracts/generated directory. These are
// committed in-tree, because it allows the crate to be built without
// depending on any extra tooling like Truffle or Solidity.

use ethcontract_generate::{Builder, ContractBindings, Source};
use heck::SnakeCase;
use std::{
  ffi::OsStr,
  path::{Path, PathBuf},
};

pub type WrappedError = Box<dyn std::error::Error>;
pub type WrappedResult<T> = std::result::Result<T, WrappedError>;

// This just finds all the .json files in a directory. Not much to see.
fn sources(input_dir: PathBuf) -> WrappedResult<Vec<PathBuf>> {
  let mut sources: Vec<PathBuf> = vec![];

  for entry in input_dir.read_dir()? {
    let path = entry?.path();
    if !path.metadata()?.is_file() {
      continue;
    }

    if let Some(extension) = path.extension() {
      if extension.eq("json") {
        sources.push(path)
      }
    }
  }

  Ok(sources)
}

// This maps sources to output files, and handles a naming issue.
fn plan(sources: Vec<PathBuf>, output_dir: PathBuf) -> WrappedResult<Vec<(Source, PathBuf)>> {
  let mut plan: Vec<(Source, PathBuf)> = vec![];

  for source in sources {
    let file_stem = source
      .file_stem()
      .expect("cannot extract stem from filename")
      .to_str()
      .expect("filename is not valid Unicode")
      .to_snake_case();
    let source = Source::Local(source.clone());
    let path = if file_stem.eq("create2") {
      output_dir.join("create_2.rs")
    } else {
      output_dir.join(format!("{}.rs", file_stem))
    };
    plan.push((source, path));
  }

  Ok(plan)
}

// This generates the code, and handles at least one signature issue.
// There's probably a better way to handle this, but there are multiple
// source files with the same issue, and this doesn't need to be fast,
// since it's not even run at build time, but manually when needed.
fn contract_bindings(source: Source) -> WrappedResult<ContractBindings> {
  match Builder::with_source(source.clone())
    .with_visibility_modifier(Some("pub"))
    .add_event_derive("serde::Deserialize")
    .add_event_derive("serde::Serialize")
    .add_method_alias(
      String::from("safeTransferFrom(address,address,uint256,bytes)"),
      String::from("safe_transfer_from_with_data"),
    )
    .generate()
  {
    Ok(contract_bindings) => Ok(contract_bindings),
    Err(_) => Ok(
      Builder::with_source(source)
        .with_visibility_modifier(Some("pub"))
        .add_event_derive("serde::Deserialize")
        .add_event_derive("serde::Serialize")
        .generate()?,
    ),
  }
}

// This generates the module that pulls in and exposes all the other
// files. It's a flat tree, because it doesn't do anything fancy like
// parsing the sources to figure out where they should go. There's a
// hand-written mod.rs that exposes these in a tree that corresponse to
// the OpenZeppelin sources though.
fn write_module(path: PathBuf, output_files: Vec<PathBuf>) -> std::io::Result<()> {
  let mut mods: Vec<String> = vec![];
  let mut uses: Vec<String> = vec![];

  for module in output_files
    .iter()
    .flat_map(|path| path.file_stem())
    .flat_map(|stem| stem.to_str())
  {
    mods.push(format!("pub mod {};", &module));
    uses.push(format!("pub use {}::*;", &module));
  }

  mods.sort();
  uses.sort();

  let clippy = String::from("#![allow(clippy::all)]");
  let mut contents = [clippy, mods.join("\n"), uses.join("\n")].join("\n\n");
  contents.push_str("\n");
  std::fs::write(path, contents)
}

// This makes no attempts to preserve modifications or anything. It will
// blithely stomp all over anything in the "generated" directory, because
// that words have meanings, and that directory has that name for a reason.
fn generate<T: AsRef<OsStr>>(input: T, output: T) -> WrappedResult<()> {
  let input_dir = Path::new(input.as_ref()).to_path_buf();
  let sources = sources(input_dir)?;

  let output_base = Path::new(output.as_ref());
  let output_file = output_base.join("generated.rs");
  let output_dir = output_base.join("generated");
  if !output_dir.exists() {
    std::fs::DirBuilder::new()
      .recursive(true)
      .create(&output_dir)?
  }

  let plan = plan(sources, output_dir)?;

  let mut output_files: Vec<PathBuf> = vec![];

  for (source, path) in plan {
    match contract_bindings(source) {
      Ok(contract_bindings) => {
        contract_bindings
          .write_to_file(&path)
          .expect("failed to write contract bindings");
        println!("SUCCESS - {:?}", &path);
        output_files.push(path);
      }
      Err(error) => println!("FAILURE - {:?} - {:?}", &path, error),
    };
  }

  output_files.sort();

  write_module(output_file, output_files)?;

  Ok(())
}

fn main() -> WrappedResult<()> {
  generate(
    "node_modules/@openzeppelin/contracts/build/contracts",
    "src/openzeppelin/contracts",
  )?;
  generate(
    "node_modules/@openzeppelin/contracts-upgradeable/build/contracts",
    "src/openzeppelin/contracts_upgradeable",
  )?;
  generate(
    "legacy/node_modules/@openzeppelin/contracts/build/contracts",
    "src/openzeppelin/contracts_2x",
  )?;

  Ok(())
}
