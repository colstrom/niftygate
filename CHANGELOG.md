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
