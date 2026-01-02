# CasperLiquid Deployment Guide

This guide covers the deployment process for the CasperLiquid staking contract on Casper Testnet.

## Prerequisites

1. **Rust and Cargo**: Ensure you have Rust installed with Cargo
2. **Odra Framework**: Install Odra CLI tools
3. **Casper Account**: You need a Casper account with testnet CSPR for deployment
4. **Secret Key**: Your account's secret key for signing transactions

## Setup Instructions

### 1. Environment Configuration

Copy the environment template and configure your settings:

```bash
cp .env.example .env
```

Edit `.env` and set your configuration:

```bash
# Required: Your Casper account secret key
SECRET_KEY=your_actual_secret_key_here

# Optional: Customize network settings
NODE_ADDRESS=http://3.143.158.19:7777
NETWORK_NAME=casper-test
CHAIN_NAME=casper-test
```

**⚠️ Security Note**: Never commit your `.env` file to version control. The `.env` file is already in `.gitignore`.

### 2. Generate or Import Secret Key

If you don't have a secret key, generate one:

```bash
casper-client keygen ./keys/
```

This creates `secret_key.pem` and `public_key.pem` files. Use the content of `secret_key.pem` as your `SECRET_KEY` in `.env`.

### 3. Verify Configuration

Run the configuration verification:

```bash
cargo run -- verify
```

This checks:
- ✅ `.env` file exists and is properly configured
- ✅ `Odra.toml` is properly set up
- ✅ Required environment variables are set
- ✅ Secret key is configured (not the placeholder)

## Deployment Process

### 1. Build the Contract

First, build the contract to ensure everything compiles:

```bash
cargo build --release
```

### 2. Deploy to Testnet

Deploy the contract using Odra:

```bash
cargo odra deploy --network casper-test
```

Or use the deployment script:

```bash
cargo run -- deploy
```

### 3. Verify Deployment

After deployment, you should see output similar to:

```
✅ Contract deployed successfully
📋 Contract Hash: hash-1234567890abcdef...
📋 Contract Package Hash: hash-abcdef1234567890...
🌐 Network: casper-test
```

**Important**: Save the contract hash - you'll need it for frontend integration.

### 4. Test the Deployment

You can test basic contract functionality:

```bash
# Check contract metadata
casper-client get-state-root-hash --node-address http://3.143.158.19:7777

# Query contract state (replace with your contract hash)
casper-client query-global-state \
  --node-address http://3.143.158.19:7777 \
  --state-root-hash <state-root-hash> \
  --key <contract-hash>
```

## Contract Configuration

The contract is deployed with the following initial configuration:

- **Name**: "Staked CSPR"
- **Symbol**: "stCSPR"  
- **Decimals**: 9 (matching CSPR)
- **Initial Supply**: 0 (no tokens minted initially)

## Troubleshooting

### Common Issues

1. **"SECRET_KEY not found"**
   - Ensure you've copied `.env.example` to `.env`
   - Set a valid secret key in the `.env` file

2. **"Invalid secret key format"**
   - Ensure your secret key is in the correct PEM format
   - Check that there are no extra spaces or characters

3. **"Insufficient funds"**
   - Ensure your account has enough testnet CSPR for deployment
   - Get testnet tokens from the Casper faucet

4. **"Node connection failed"**
   - Check that the node address is correct and accessible
   - Try using a different Casper testnet node

### Getting Testnet CSPR

To deploy on testnet, you need testnet CSPR tokens:

1. Visit the [Casper Testnet Faucet](https://testnet.cspr.live/tools/faucet)
2. Enter your public key hash
3. Request testnet tokens
4. Wait for the tokens to arrive (usually a few minutes)

### Useful Commands

```bash
# Check account balance
casper-client get-balance \
  --node-address http://3.143.158.19:7777 \
  --public-key <your-public-key.pem>

# Check deployment status
casper-client get-deploy \
  --node-address http://3.143.158.19:7777 \
  <deploy-hash>

# List available Odra commands
cargo odra --help
```

## Next Steps

After successful deployment:

1. **Save Contract Details**: Record the contract hash and package hash
2. **Update Frontend**: Configure the frontend with the deployed contract hash
3. **Test Integration**: Test the complete stake/unstake flow
4. **Monitor Contract**: Set up monitoring for contract events and state

## Security Considerations

- **Private Keys**: Never share or commit your secret keys
- **Testnet Only**: This configuration is for testnet deployment only
- **Gas Limits**: Monitor gas usage and adjust limits if needed
- **State Validation**: Always verify contract state after deployment

For additional support, refer to the [Odra Documentation](https://odra.dev) and [Casper Documentation](https://docs.casper.network).