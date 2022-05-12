# 0.x (unreleased)

- (internal) update rust edition to 2021
- (dependency) camino 1.0.7 -> 1.0.8
- (dockerfile) added aarch64 support
- (dockerfile) added static linking

# 0.8.0

- (feature) crowdfunding support (experimental)
- (feature) marketplace support (experimental)
- (feature + commandline) added "asset" subcommand
  - (package) this is also available standalone via the "niftygate-asset" crate.
  - provides discovery of Solidity compiler versions.
  - can download any version of Solidity with no external tooling required.
    - provides transparent on-disk compression for cached assets.
      - supports LZ4, Deflate (gzip), and Brotli, depending on your needs.
    - integrity verification, to facilitate secure supply chains.
  - flexible caching policies to support environments like CI/CD, air-gapped, or immutable deployments.
  - granular network access policies to reduce attack surface when downloading.
  - supports custom mirrors, for trusted build environments.
  - respects various platform-specific standards for paths
  - automation and scripting-friendly.
    - provides built-in tooling for cache maintenance (usage reporting, cleanup, etc).
    - built-in filtering with support for Regular Expressions and Semantic Versioning constraints.
  - user-friendly interative prompts to guide you (optional, of course).
  - (experimental) able to extract embedded WebAssembly from soljson release artifacts.
  - (unsupported) built-in benchmarking of supported compression schemes.
  - (unsupported) built-in tooling for analyzing soljson release artifacts.
- (fix) CLI enforces required arguments for TLS support.
- (internal) replace a fallible conversion with an infallible one.
- (feature) added TLS support to embedded demo application.
- (package) "guide" subcommand moved into "niftygate-guide" crate.
- (package) "contract" subcommand moved into "niftygate-contract" crate.
- (niftygate-certificate) now with more accurate help text!
- (commandline) "units" subcommand can now convert values.
  - this introduces a "units convert" subcommand for use in scripting.
  - the previous functionality is still available via "units show".
- (library) BalanceScale now implements `Copy`.
- (deprecation) WrappedError is deprecated, use anyhow::Error.
- (deprecation) WrappedResult is deprecated, use anyhow::Result.
- (deprecation) command::run() is deprecated, use command::Command.execute().
- (deprecation) command::HexData is deprecated as an import path. It has been moved to the crate root.
- (breaking) rename command::Options to command::Command.
- (breaking) command::Options::Web3 enum variant moved to struct command::web3::Command.
- (breaking) command::Options::Demo enum variant moved to struct command::demo::Command.
- (breaking) command::Options::Units enum variant moved to struct command::units::Command.
- (breaking: niftygate-certificate) command::Generate takes time::OffsetDateTime now, now chrono::DateTime<Utc>
  - this is due to a change in a dependency, not an active choice to move away from Chrono.
- (dependency) anyhow 1.0.43 -> 1.0.57
- (dependency) async-std 1.10.0 -> 1.11.0
- (dependency) async-trait 0.1.51 -> 0.1.53
- (dependency) blake3 1.2.0 -> 1.3.1
- (dependency) brotli 3.3.2 -> 3.3.4
- (dependency) console 0.14.1 -> 0.15.0
- (dependency) dialoguer 0.9.0 -> 0.10.1
- (dependency) digest 0.10.1 -> 0.10.3
- (dependency) ethcontract 0.15.3 -> 0.17.0
- (dependency) ethcontract-generate 0.15.3 -> 0.17.0
- (dependency) heck 0.3.3 -> 0.4.0
- (dependency) pem 0.8.3 -> 1.0.2
- (dependency) rcgen 0.8.13 -> 0.9.2
- (dependency) regex 1.5.4 -> 1.5.5
- (dependency) secp256k1 0.20.3 -> 0.21.3
- (dependency) semver 1.0.4 -> 1.0.9
- (dependency) serde 1.0.130 -> 1.0.137
- (dependency) serde_json 1.0.67 -> 1.0.81
- (dependency) structopt 0.3.23 -> 0.3.26
- (dependency) strum 0.21.1 -> 0.24.0
- (dependency) surf 2.3.1 -> 2.3.2
- (dependency) thiserror 1.0.29 -> 1.0.31
- (dependency) wasm-bindgen-futures 0.4.28 ->? 0.4.29
- (dependency) web3 0.17.0 -> 1.18.0
- (dependency) x509-parser 0.11.0 -> 0.13.1
- (dependency) xdg 2.4.0 -> 2.4.1

# 0.7.0

- (feature) add TLS support!
- (commandline) added --with-tls option.
- (commandline) added --tls-certificate-path option.
- (commandline) added --tls-key-path option.
- (feature + commandline) added "certificate" subcommand.
  - (package) this is also available standalone via the "niftygate-certificate" crate.
  - can generate self-signed certificates, CA certs, and can sign certs.
  - this provides a way to generate and sign certificates without external tools.
  - includes support for constrained authorities
    - can restrict the number of intermediate authorities allowed.
    - can limit the scope of the authority based on DNS, IP, Directory Name, or Email.
    - constraints can be combined, and exclusions are supported.
- (lint) fixed collapsible_if(s)
- (lint) fixed needless_borrow(s)
- (lint) fixed redundant_static_lifetimes(s)
- (depencency) added anyhow
- (dependency) added thiserror
- (dependency) added tide-rustls

# 0.6.3

- (dependency) async-std 1.9.0 -> 1.10.0
- (dependency) ethcontract 0.12.2 -> 0.15.3
- (dependency) ethcontract-generate 0.12.2 -> 0.15.3
- (dependency) serde 1.0.129 -> 1.0.130
- (dependency) serde_json 1.0.66 -> 1.0.67
- (dependency) structopt 0.3.22 -> 0.3.23
- (dependency) surf 2.2.0 -> 2.3.1
- (dependency) web3 0.16.0 -> 0.17.0
- (internal) regenerate bindings

# 0.6.2

- (internal) split into packages.
  - niftygate is the main package, containing the app and library.
  - niftygate-bindings contains the bindings for various smart contracts.
  - niftygate-bindgen contains the code generator for the contract bindings.
  - there should be no impact to compatibility with this change.
  - this resolves an ongoing challenge with dependency upgrades.

# 0.6.1

- (library) updated bindings for OpenZeppelin contracts to 4.3.1 - see [Changelog](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v4.3/CHANGELOG.md) for details.
- (internal) added support for (I)GovernorCompatiblityBravo contracts to generator.
- (internal) added support for IERC1363 contracts to generator.
- (internal) removed unfinished support for legacy contract bindings from generator.

# 0.6.0

- (breaking) renamed from sig-proxy to niftygate.
- (library) updated bindings for OpenZeppelin contracts to 4.2.0 - see [Changelog](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v4.2/CHANGELOG.md) for details.
  - (deprecation) ERC20FlashMint no longer a draft extension.
    - update import path by removing `draft::` to fix.
- (dependency) secp256k1 0.20.2 -> 0.20.3
- (dependency) structopt 0.3.21 -> 0.3.22

# 0.5.0

- (internal) added support for upgradeable contracts to generator.
- (library) added generated bindings for upgradeable OpenZeppelin contracts.

# 0.4.1

- (fix) clearer error message when asked to provide signatures, but no secret key is given.

# 0.4.0

- (breaking) Contract methods that took Vec<u8> now take Bytes<Vec<u8>>
- (breaking) ethcontract::Void was removed, replaced with ()
- (dependency) ethcontract 0.11.3 -> 0.12.2
- (dependency) ethcontract-generate 0.11.3 -> 0.12.2
- (dependency) heck 0.3.2 -> 0.3.3
- (dependency) strum 0.20.0 -> 0.21.0
- (dependency) web3 0.15.0 -> 0.16.0

# 0.3.1

- (commandline) added help text to the `events` subcommand.
- (lint) disabled clippy lints for generated code.
- (lint) replaced some `.and_then(|x| Ok(y))` with `.map(|x| y)`.
- (lint) stopped calling `clone()` on `Copy` types.

# 0.3.0

- (breaking + library) changed type of ProvidesAccountVerification.web3 from Web3<WebSocket> to Web3<DynTransport>
- (breaking + library) changed type of BalanceRequirement.web3 from Web3<WebSocket> to Web3<DynTransport>
- (breaking + library) changed type of ProvidesSignature.web3 from Web3<WebSocket> to Web3<DynTransport>
- (internal) added command::contract::dump module for console output.
- (internal) added util::web3_from_url for converting URLs.
- (internal) everything uses DynTransport now.
- (internal) use exported web3 from ethcontract.
- (library) contract events are (de)serializable now.
- (commandline) added tools to query and stream events for deployed contracts.
- (dependency) add serde
- (dependency) add serde_json

# 0.2.0

- (fix) stopped counterintuitively parsing U256 CLI options as hex
- (internal) added program to generate bindings for OpenZeppelin contracts
- (library) added generated bindings for OpenZeppelin contracts
- (library) added surf, ethcontract, ethabi to prelude
- (library + commandline) added support for ERC20 tokens
- (library + commandline) added support for ERC721 tokens
- (library + commandline) added support for ERC777 tokens
- (library + commandline) added support for ERC1155 tokens
- (commandline) added interactive guide to choose a token type
- (commandline) added tools to deploy embedded contracts
- (commandline) added tools to interact with deployed contracts
- (dependency) secp256k1 0.20.1 -> 0.20.2
- (dependency) added dialoguer
- (dependency) added console
- (dependency) added ethcontract
- (dependency) changed features for surf and tide to omit non-Rust dependencies

# 0.1.2

- Forcing a fixed terminal width is rude. Stopped doing that.

# 0.1.1

- Fixed a bug where an extra `surf::Client` was allocated on every
  proxied request. Thanks to @jbr for catching this.

# 0.1.0 - Initial Release
