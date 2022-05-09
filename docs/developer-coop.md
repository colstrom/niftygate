# Scenario: Developer Co-op.

Difficulty: Complex

# Requirements

- IPFS

# Assumptions

- There is a single seller.
- There is a single buyer.
- There are five members that receive royalties from sales.
- 5% of ALL sales are paid as royalties.

# Setup

## Environment

```fish
set SELLER_ADDRESS 0x24f9F97C9e540fed57EF81f6c9aEAbdB6Fc73ACd
set SELLER_PRIVATE_KEY ab0418d60f2207c30c57b0ff0e8678a48e064363099314e49473074aea45c4d7

set BUYER_ADDRESS 0x0c8Daf0Cc1B1603093184ed54F9Cd7E97Ec63a11
set BUYER_PRIVATE_KEY 25d9d96bd84739c17a690a56fcb45d6500d4ffbeead695ce452123fc358a431e

set ROYALTY_PAYEE_ADDRESSES \
  0x73aa86C6164a4d62be618DE1fd20e3F8E492A2e4 \
  0x703859bcDE68a415D0fBFd701254fe4E74533701 \
  0x358E66b4664Ac1749E1Fb3DF1043b1B4f2ba6518 \
  0x5Ef149226dE6DCC1C07d5920643b07F8CAB4C045 \
  0x501bF988EE738DFAC112eF9EDC2862632830A26c

```

## Royalties Payment

### Deploy Smart Contract

```fish
niftygate contract deploy experimental RoyaltiesPayment

# Deployed at 0x591b162308f07673743a0dbfb192e9b0bf935d91

set --export ROYALTIES_PAYMENT_CONTRACT 0x591b162308f07673743a0dbfb192e9b0bf935d91
```

### Add Payees

```fish
set --export CONTRACT_ADDRESS $ROYALTIES_PAYMENT_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

for payee in $ROYALTY_PAYEE_ADDRESSES
  niftygate contract experimental RoyaltiesPayment addPayee \
    --payee $payee
end

# Success (Transaction 0xe561162e52b53c764fc2128d26ef0c1f4c0822cc59619aeff68ebe612f7fddc3)
# Success (Transaction 0x8731cb7e236c7f5029fce2a302afd31e1d90cba53a71a080f27f16d7ee9d4bd0)
# Success (Transaction 0x30a8ca6f598098bd032e3e4b67d7f0df292dee833dc11c56cb275e133a614320)
# Success (Transaction 0x2990bd3c2e909a66da5221bc39147fef3ee4da855c9e6faf6134a69aa15acb89)
# Success (Transaction 0x8e16a753b1956a6a65d62fb8ac081059853afb50854b8e57182a72dcbe0d7730)
```

## Token (NFT as a Product)

### Deploy Smart Contract

```fish
niftygate contract deploy experimental Token \
  --initialRoyaltiesReceiver $ROYALTIES_PAYMENT_CONTRACT

# Deployed at 0x2ef54709af33b17825e174d8c785b147103f1024

set --export PRODUCT_TOKEN_CONTRACT 0x2ef54709af33b17825e174d8c785b147103f1024
```

### Mint Tokens

```fish
set --export CONTRACT_ADDRESS $PRODUCT_TOKEN_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

for x in (seq 5)
  set IPFS_HASH 0xDEADBEEF-$x
  niftygate contract send experimental Token mint \
    --recipient $SELLER_ADDRESS \
    --hash $IPFS_HASH
end

# Success (Transaction 0x4ad06053e403e12955068f363d0dd65b73ebf7507fce6e3883a918ed3d866fbe)
# Success (Transaction 0xc50f23493ac7f46fc3aa813da65c381e4fc1da8c734ea97aa69e8fa505a3d1a4)
# Success (Transaction 0xaaeb1c5c964b68b9b0f3b1eecc5c43246d4782b2b24af5e99483fbf0208a48df)
# Success (Transaction 0xa5537124b3a999c2d1ea4eb606ecf13f265f1fe7183fb87f8c1a396240f15cd4)
# Success (Transaction 0x1fe0dfd2509094f948fd75b2a844c4a810a90c89d1ab2cfd866a391e70aa4b8d)
```

## Marketplace

### Deploy Smart Contract

```fish
niftygate contract deploy experimental Marketplace \
  --tokenContractAddress $PRODUCT_TOKEN_CONTRACT

# Deployed at 0xecf84e809861fce6e50f9b58eb02c526d50684be

set --export MARKETPLACE_CONTRACT 0xecf84e809861fce6e50f9b58eb02c526d50684be
```

### Approve the Marketplace to Transfer the Tokens.

```fish
set --export CONTRACT_ADDRESS $PRODUCT_TOKEN_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

for tokenId in (seq 5)
  niftygate contract send experimental Token approve \
    --to $MARKETPLACE_CONTRACT \
    --tokenId $tokenId
end

# Success (Transaction 0x2a0b29c569d699771e5e7a1e8246a607276125228b0cae2e1a5ec862f4cf5b03)
# Success (Transaction 0x736b17fce179ae9af81af665eaa2fed6f6a1e52fbd2b5cd6071b46d5e60b163b)
# Success (Transaction 0x9ba6b400813ad6024bec176c2dde1a20010f6348cbf04353acedcc10aeffe598)
# Success (Transaction 0x78eea6fe4ee4fd2c66d65cccf632a831043234d1ccd1effca66fdc7281a5f5c8)
# Success (Transaction 0x6ce6c50b15ab9143cae8df1057b7dfec227af481da54861014b061f8a990ce99)
```

# Usage

## As a buyer, offer 50 ETH for each token.

```fish
set --export CONTRACT_ADDRESS $MARKETPLACE_CONTRACT
set --export PRIVATE_KEY $BUYER_PRIVATE_KEY

for tokenId in (seq 5)
  niftygate contract send experimental Marketplace makeBuyOffer \
    --tokenId $tokenId \
    --value 50000000000000000000
end

# Success (Transaction 0xd9969aa7dc142bcc767d099b3c0d2a2787e330ddc5679dfafb9202a75c8916dc)
# Success (Transaction 0x0f935541067670980fda63f5180e1cc7ef6b72b12a1e6484fecf66ceb004904d)
# Success (Transaction 0x845940c06c9d4c7a41772ce6982d797c4671ce11e202ba3f568eaf8bb0cec03d)
# Success (Transaction 0x1141f90325285916367c2e00916015414b904aa599da6a81607f79e24677891c)
# Success (Transaction 0xde97f5bb662320129712cbc6c267bd46e6579aceed50b2d4dcf3e9ee73e7321b)
```

## As the seller, accept the offers.

```fish
set --export CONTRACT_ADDRESS $MARKETPLACE_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

for tokenId in (seq 5)
  niftygate contract send experimental Marketplace acceptBuyOffer \
    --tokenId $tokenId
end

# Success (Transaction 0xec0cf2217fbfa6f14c07ae87ee7b35b7096b19c50e1f8f0619553094da6cfb25)
# Success (Transaction 0xdf93961b9e45734c40a9f124ac66f27e968c3c002858b0c0f87757c3fcb08b1f)
# Success (Transaction 0xbc4a00629970153b7f686544c2cfdb71a35b12147de9a5b8bbd315a48c495ab4)
# Success (Transaction 0x8959f9ad97fa91465e5309f6793737a362706d913fb1414b229ea880c7f26171)
# Success (Transaction 0x1209309eab57de4492cd67b10783a63dab334bf5a36acb7e78232b0ff7229bfc)
```

## Pay everyone their cut

```fish
set --export CONTRACT_ADDRESS $ROYALTIES_PAYMENT_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

niftygate contract send experimental RoyaltiesPayment payAll

# Success (Transaction 0x4797a31050fffbd692d94cfe3fd13e465e553f4592478114c28f5f3b28d085aa)
```

# Bonus Round

It's also possible to pay users out in ERC20-compliant token. This is important,
because it allows stablecoins like USDT, USDC, and DAI. It can also be used for
paying out in an internal currency, as many applications do.

## Use an In-App Currency

### Deploy Smart Contract

```fish
niftygate contract deploy ERC20PresetMinterPauser \
  --name "Gems" \
  --symbol GEM

# Deployed at 0xa49a3a81de1e567e23a302cf770d7eb6b7fbb82e

set --export IN_APP_CURRENCY_CONTRACT 0xa49a3a81de1e567e23a302cf770d7eb6b7fbb82e
```

## Mint Some Tokens

```fish
set --export CONTRACT_ADDRESS $IN_APP_CURRENCY_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

niftygate contract send ERC20PresetMinterPauser mint \
  --to $ROYALTIES_PAYMENT_CONTRACT \
  --amount 1000000000000000000000

# Success (Transaction 0x4ed898e4f8aca390e576141fcca6d299ce5abb10a85e0e7eabefba9c3c866a66)
```

## Pay Out

```fish
set --export CONTRACT_ADDRESS $ROYALTIES_PAYMENT_CONTRACT
set --export PRIVATE_KEY $SELLER_PRIVATE_KEY

niftygate contract send experimental RoyaltiesPayment withdrawErc20 \
  --token $IN_APP_CURRENCY_CONTRACT

# Success (Transaction 0xad7e60f3dd9ec848dcc86bdd55327b38075bb83724f44d9fdff9b8130b96fe1b)
```

## Check Balances

```fish
set --export CONTRACT_ADDRESS $IN_APP_CURRENCY_CONTRACT

for account in $ROYALTY_PAYEE_ADDRESSES
  niftygate contract call ERC20PresetMinterPauser balanceOf \
    --account $payee
end

# 200000000000000000000
# 200000000000000000000
# 200000000000000000000
# 200000000000000000000
# 200000000000000000000
```
