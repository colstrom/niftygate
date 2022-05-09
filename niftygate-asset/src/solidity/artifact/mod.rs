pub mod extract;
pub(crate) mod overview;

#[allow(clippy::module_inception)]
mod artifact;
mod emscripten;
mod wasm;

pub use artifact::{ArtifactError, ReleaseArtifact};
pub use wasm::WasmArtifact;
