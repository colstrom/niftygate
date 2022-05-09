use resast::prelude::*;
use ressa::{CommentHandler, Parser};
use semver::Version;
use std::borrow::Cow;

/// The WASM binary blob extracted from a soljson release artifact.
///
/// Depending on the version of Solidity (due to changes in build process),
/// this may or may not be compressed.
pub struct ExtractedWasm<'source> {
  data: Cow<'source, str>,
  size: usize,
  compressed: bool,
}

impl<'source> ExtractedWasm<'source> {
  /// The raw binary blob. It's probably base64-encoded.
  pub fn data(&self) -> &str {
    self.data.as_ref()
  }

  /// Size of the decompressed WASM, in bytes.
  pub fn uncompressed_size(&self) -> usize {
    self.size
  }

  /// Is the blob compressed? If so, it's probably LZ4.
  pub fn is_compressed(&self) -> bool {
    self.compressed
  }
}

/// Extracts WASM binary blobs from soljson release artifacts, using a
/// version-specific algorithm.
pub struct WasmExtractor {
  algorithm: Algorithm,
}

impl WasmExtractor {
  pub fn extract_wasm<'program, CH>(
    &self,
    parser: Parser<'program, CH>,
  ) -> Option<ExtractedWasm<'program>>
  where
    CH: CommentHandler<'program>,
  {
    self.algorithm.extract_wasm(parser)
  }
}

impl<T> From<T> for WasmExtractor
where
  T: Into<Algorithm>,
{
  fn from(coercible: T) -> Self {
    let algorithm = coercible.into();
    Self { algorithm }
  }
}

/// Artifact layout changed in this version, due to Emscripten upgrades in
/// the Solidity compiler build process.
pub const V0_8_10: &Version = &Version::new(0, 8, 10);

/// Artifact layout changed significantly in this version, due to modified
/// Emscripten build flags in the Solidity compiler build process.
pub const V0_8_12: &Version = &Version::new(0, 8, 12);

/// The algorithm used to extract WASM from soljson release artifacts.
#[non_exhaustive]
enum Algorithm {
  Legacy(Legacy),
  Modern(Modern),
  ModernCompressed(ModernCompressed),
}

impl<V> From<V> for Algorithm
where
  V: AsRef<Version>,
{
  fn from(version: V) -> Self {
    let version = version.as_ref();

    if version >= V0_8_12 {
      Self::ModernCompressed(ModernCompressed)
    } else if version >= V0_8_10 {
      Self::Modern(Modern)
    } else {
      Self::Legacy(Legacy)
    }
  }
}

/// Extracts WASM from soljson release artifacts.
trait ExtractWasm<'program, CH>
where
  CH: CommentHandler<'program>,
{
  fn extract_wasm(&self, parser: Parser<'program, CH>) -> Option<ExtractedWasm<'program>>;
}

impl<'program, CH> ExtractWasm<'program, CH> for Algorithm
where
  CH: CommentHandler<'program>,
{
  fn extract_wasm(&self, parser: Parser<'program, CH>) -> Option<ExtractedWasm<'program>> {
    match self {
      Self::Legacy(algorithm) => algorithm.extract_wasm(parser),
      Self::Modern(algorithm) => algorithm.extract_wasm(parser),
      Self::ModernCompressed(algorithm) => algorithm.extract_wasm(parser),
    }
  }
}

/// Works for everything up to and including Solidity 0.8.9
struct Legacy;

impl<'program, CH> ExtractWasm<'program, CH> for Legacy
where
  CH: CommentHandler<'program>,
{
  fn extract_wasm(&self, parser: Parser<'program, CH>) -> Option<ExtractedWasm<'program>> {
    for program_part in parser.flatten() {
      if let ProgramPart::Decl(Decl::Var(kind, vars)) = program_part {
        if kind == VarKind::Var {
          for var in vars {
            if let Pat::Ident(ident) = var.id {
              if ident.name.eq("wasmBinaryFile") {
                if let Some(Expr::Lit(Lit::String(StringLit::Double(source)))) = var.init {
                  let size = source.len();
                  return Some(ExtractedWasm {
                    data: source,
                    size,
                    compressed: false,
                  });
                }
              }
            }
          }
        }
      }
    }

    None
  }
}

/// Works for Solidity 0.8.10 - 0.8.11
struct Modern;

impl<'program, CH> ExtractWasm<'program, CH> for Modern
where
  CH: CommentHandler<'program>,
{
  fn extract_wasm(&self, parser: Parser<'program, CH>) -> Option<ExtractedWasm<'program>> {
    for program_part in parser.flatten() {
      if let ProgramPart::Stmt(Stmt::Expr(Expr::Assign(assign))) = program_part {
        if assign.operator == AssignOp::Equal {
          if let AssignLeft::Expr(left) = assign.left {
            if let Expr::Ident(ident) = *left {
              if ident.name.eq("wasmBinaryFile") {
                if let Expr::Lit(Lit::String(StringLit::Double(source))) = *assign.right {
                  let size = source.len();
                  return Some(ExtractedWasm {
                    data: source,
                    size,
                    compressed: false,
                  });
                }
              }
            }
          }
        }
      }
    }

    None
  }
}

/// Works for 0.8.12 - 0.8.13 (the latest release at the time of writing)
///
/// Should probably work with newer versions, but unable to verify this
/// without violating causality (cannot test versions before they exist).
struct ModernCompressed;

impl<'program, CH> ExtractWasm<'program, CH> for ModernCompressed
where
  CH: CommentHandler<'program>,
{
  fn extract_wasm(&self, parser: Parser<'program, CH>) -> Option<ExtractedWasm<'program>> {
    for program_part in parser.flatten() {
      if let ProgramPart::Stmt(Stmt::Expr(Expr::Assign(assign))) = program_part {
        if assign.operator == AssignOp::Equal {
          if let AssignLeft::Expr(left) = assign.left {
            if let Expr::Member(member) = *left {
              if let Expr::Ident(object) = *member.object {
                if object.name.eq("Module") {
                  if let Expr::Lit(Lit::String(StringLit::Double(property))) = *member.property {
                    if property.eq("wasmBinary") {
                      if let Expr::Call(call) = *assign.right {
                        if let Expr::Func(func) = *call.callee {
                          let mut source = None;
                          let mut size = None;

                          for (index, argument) in call.arguments.into_iter().enumerate() {
                            if let Some(FuncArg::Pat(Pat::Ident(Ident { name }))) =
                              func.params.get(index)
                            {
                              match (argument, name.as_ref()) {
                                (Expr::Lit(Lit::String(StringLit::Double(value))), "source") => {
                                  source = Some(value);
                                }
                                (Expr::Lit(Lit::Number(value)), "uncompressedSize") => {
                                  if let Ok(value) = value.parse::<usize>() {
                                    size = Some(value);
                                  }
                                }
                                _ => {}
                              }
                            }
                          }

                          if let (Some(source), Some(size)) = (source, size) {
                            return Some(ExtractedWasm {
                              data: source,
                              size,
                              compressed: true,
                            });
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    None
  }
}
