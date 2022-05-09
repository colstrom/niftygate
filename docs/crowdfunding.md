# Scenario: Crowdfunding

## Requirements

These are out of scope for NiftyGate. You need to solve these on your own.

  - You have a site already.
  - You have a way of collecting funds.
  - You have a wallet.
  - You have wallet addresses for your backers.

## Assumptions in this scenario.

These are not requirements, they are just the assumptions used in the examples.

  - You are raising $100,000.
  - You are using NiftyGate's embedded contracts.
  - You are issuing tokens on a 1-for-1 basis.
  - The token supply is fixed (100,000 tokens).
  - Your site is (internally) reachable at `http://site.internal`.
  - Your site is (externally) reachable at `https://backers.project.local`.
    - This URL resolves to NiftyGate.
  - Your wallet address is `0x24f9F97C9e540fed57EF81f6c9aEAbdB6Fc73ACd`.
  - Your project has been funded by five generous backers with the following addresses:
    - `0x0c8Daf0Cc1B1603093184ed54F9Cd7E97Ec63a11`
    - `0x73aa86C6164a4d62be618DE1fd20e3F8E492A2e4`
    - `0x703859bcDE68a415D0fBFd701254fe4E74533701`
    - `0x358E66b4664Ac1749E1Fb3DF1043b1B4f2ba6518`
    - `0x5Ef149226dE6DCC1C07d5920643b07F8CAB4C045`
  - Each of your backers has contributed an equal amount ($20,000).
  - You are using `fish` as your shell.
    - Command syntax may need to be adjusted slightly for `bash` and other vintage shells.

## Process

## Environment
```fish

set OPERATOR_ADDRESS 0x24f9F97C9e540fed57EF81f6c9aEAbdB6Fc73ACd
set BACKERS \
  0x0c8Daf0Cc1B1603093184ed54F9Cd7E97Ec63a11 \
  0x73aa86C6164a4d62be618DE1fd20e3F8E492A2e4 \
  0x703859bcDE68a415D0fBFd701254fe4E74533701 \
  0x358E66b4664Ac1749E1Fb3DF1043b1B4f2ba6518 \
  0x5Ef149226dE6DCC1C07d5920643b07F8CAB4C045
```

### Deploy an ERC20 Smart Contract

```fish
niftygate contract deploy ERC20PresetFixedSupply \
  --name "Project Backer Tokens" \
  --symbol PBT \
  --initialSupply 100000 \
  --owner $OPERATOR_ADDRESS

# Deployed at 0xf1f64ff35823f804326931519d58b5d66be28dc7

set --export TOKEN_CONTRACT 0xf1f64ff35823f804326931519d58b5d66be28dc7
```

### Collect your funds from your backers however you like.

This step is out of scope for NiftyGate.

### Issue tokens to your backers.

```fish
set --export CONTRACT_ADDRESS $TOKEN_CONTRACT

for backer in $BACKERS
  niftygate contract send ERC20PresetFixedSupply transfer \
    --recipient "$backer" \
    --amount 20000
end

# Success (Transaction 0x42a799a032c26d69bcfe4345c54884565840842a9fe804a6407b86f9a2c64b73)
# Success (Transaction 0x86f346f95b7e7740f9fa98e66ee7b7045983cb013978fcce64cff391485ceeb1)
# Success (Transaction 0xa5dd7b91cf8b5d91264046ada02760bdb0d938f24a3d49efa2cf4d80967dcdb4)
# Success (Transaction 0x1398e5011f55c44863f34fdebca15c2799ded37bc7bcbafe0c22dbcd6840277e)
# Success (Transaction 0xc5e54e1e2c4145bcccf1241601a249acc7bd8a3357415c192b5439383ca8ddab)
```

### Generate a TLS Certificate (if you don't already have one)

```fish
niftygate certificate generate \
  --distinguished-name "CN=backers.project.local" \
  --out-certificate "tls.crt" \
  --out-key "tls.key"
```

### Run NiftyGate

```fish
niftygate web3 \
  --provides-account-verification \
  --with-tls \
  --tls-certificate-path "tls.crt" \
  --tls-key-path "tls.key"
  --backend "http://site.internal" \
  --challenge "I backed this project." \
  --erc20-contract-address "$TOKEN_CONTRACT" \
  --erc20-balance-minimum 1
```

### Congratulations!

If anyone accesses the site at `https://backers.project.local`, they will be
prompted to sign a message ("I backed this project.", per the configuration
above) with their wallet. This is a one-click process under most circumstances.

If the user's wallet contains at least one backer token, they can access the
gated resource. Otherwise, they cannot.
