# Scenario - Crowdsale

- You wish to raise 4000 ETH in capital.

# Environment

```fish
set OPERATOR_ADDRESS 0x24f9F97C9e540fed57EF81f6c9aEAbdB6Fc73ACd
set OPERATOR_PRIVATE_KEY ab0418d60f2207c30c57b0ff0e8678a48e064363099314e49473074aea45c4d7
```

# Deploy a Token

```fish
set --export PRIVATE_KEY $OPERATOR_PRIVATE_KEY

niftygate contract deploy ERC20PresetMinterPauser --name "Gems" --symbol GEM

# Deployed at 0x2120385fec101dab5371ed6bb853322214240ae2

set --export TOKEN_ADDRESS 0x2120385fec101dab5371ed6bb853322214240ae2
```

# Deploy a Crowdsale

```fish
niftygate contract deploy experimental Crowdsale \
  --goal 4000000000000000000 \
  --cap 5000000000000000000 \
  --rate 1000000000000000 \
  --token $TOKEN_ADDRESS \
  --openingTime "2021-12-08T23:05:00Z" \
  --closingTime "2021-12-09T00:00:00Z" \
  --wallet $OPERATOR_ADDRESS

# Deployed at 0x2f05dd2811e8e630d00f34a18f0fd9f27e4d71a7

set --export CROWDSALE_ADDRESS 0x2f05dd2811e8e630d00f34a18f0fd9f27e4d71a7
```
