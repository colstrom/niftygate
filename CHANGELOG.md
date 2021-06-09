# 0.4.1
  - (fix) clearer error message when asked to provide signatures, but no secret key is given.

# 0.4.0
  - (breaking) - Contract methods that took Vec<u8> now take Bytes<Vec<u8>>
  - (breaking) - ethcontract::Void was removed, replaced with ()
  - (dependency) - ethcontract 0.11.3 -> 0.12.2
  - (dependency) - ethcontract-generate 0.11.3 -> 0.12.2
  - (dependency) - heck 0.3.2 -> 0.3.3
  - (dependency) - strum 0.20.0 -> 0.21.0
  - (dependency) - web3 0.15.0 -> 0.16.0

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
