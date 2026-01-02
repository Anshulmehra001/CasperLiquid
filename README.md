# CasperLiquid

A liquid staking protocol for the Casper Network built with the Odra Framework.

## Overview

CasperLiquid allows users to stake CSPR tokens and receive liquid staking tokens (stCSPR) at a 1:1 ratio, enabling staking rewards while maintaining liquidity.

## Features

- Stake CSPR tokens and receive stCSPR tokens (1:1 ratio)
- Unstake stCSPR tokens to get CSPR back
- CEP-18 compliant token standard
- Web interface for easy interaction
- Deployed on Casper Testnet

## Project Structure

```
casper-liquid/
├── src/
│   └── lib.rs          # Main contract implementation
├── bin/
│   └── main.rs         # Deployment binary
├── scripts/
│   ├── deploy.sh       # Unix deployment script
│   └── deploy.ps1      # PowerShell deployment script
├── Cargo.toml          # Rust dependencies
├── Odra.toml          # Odra configuration
├── .env.example       # Environment template
├── DEPLOYMENT.md      # Detailed deployment guide
└── README.md          # This file
```

## Quick Start

### 1. Setup Environment

```bash
# Copy environment template
cp .env.example .env

# Edit .env and set your SECRET_KEY
# Get your secret key from Casper Wallet or generate with:
# casper-client keygen ./keys/
```

### 2. Verify Configuration

```bash
# Using the deployment binary
cargo run -- verify

# Or using PowerShell script (Windows)
.\scripts\deploy.ps1 -Verify

# Or using bash script (Unix/Linux/macOS)
./scripts/deploy.sh
```

### 3. Deploy to Testnet

```bash
# Using Odra directly
cargo odra deploy --network casper-test

# Or using the deployment binary
cargo run -- deploy

# Or using PowerShell script (Windows)
.\scripts\deploy.ps1

# Or using bash script (Unix/Linux/macOS)
./scripts/deploy.sh
```

## Development

### Prerequisites

- Rust 1.70+
- Odra Framework
- Casper Wallet for testing
- Testnet CSPR tokens (get from [faucet](https://testnet.cspr.live/tools/faucet))

### Building

```bash
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run property-based tests
cargo test --features proptest

# Run specific test
cargo test test_stake_functionality
```

### Local Development

```bash
# Build and run deployment tool
cargo run -- help

# Verify configuration
cargo run -- verify

# Deploy to testnet
cargo run -- deploy
```

## Deployment

For detailed deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md).

### Quick Deployment

1. **Setup**: Copy `.env.example` to `.env` and configure your `SECRET_KEY`
2. **Verify**: Run `cargo run -- verify` to check configuration
3. **Deploy**: Run `cargo run -- deploy` or `cargo odra deploy --network casper-test`
4. **Save**: Record the contract hash for frontend integration

### Deployment Scripts

- **PowerShell** (Windows): `.\scripts\deploy.ps1`
- **Bash** (Unix/Linux/macOS): `./scripts/deploy.sh`
- **Rust Binary**: `cargo run -- deploy`
- **Direct Odra**: `cargo odra deploy --network casper-test`

## Usage

The contract provides the following main functions:

### Core Functions
- `stake(amount)` - Stake CSPR tokens and receive stCSPR
- `unstake(amount)` - Burn stCSPR tokens and receive CSPR
- `total_supply()` - Get total amount of stCSPR in circulation

### CEP-18 Token Functions
- `balance_of(address)` - Check stCSPR token balance
- `transfer(recipient, amount)` - Transfer stCSPR tokens
- `approve(spender, amount)` - Approve spending allowance
- `transfer_from(owner, recipient, amount)` - Transfer on behalf
- `allowance(owner, spender)` - Check spending allowance

### Metadata Functions
- `name()` - Returns "Staked CSPR"
- `symbol()` - Returns "stCSPR"
- `decimals()` - Returns 9 (matching CSPR)

## Configuration

### Environment Variables (.env)

```bash
# Required
SECRET_KEY=your_secret_key_here

# Optional (defaults provided)
NODE_ADDRESS=http://3.143.158.19:7777
NETWORK_NAME=casper-test
CHAIN_NAME=casper-test
GAS_PRICE=1
TTL=30m
```

### Odra Configuration (Odra.toml)

The contract is configured with:
- **Network**: Casper Testnet
- **Gas Price**: 1 mote
- **TTL**: 30 minutes
- **Initial Supply**: 0 stCSPR tokens

## Security

- **Testnet Only**: This configuration is for testnet deployment
- **Private Keys**: Never commit `.env` file or share secret keys
- **Input Validation**: All user inputs are validated
- **Atomic Operations**: State changes are atomic
- **Reentrancy Protection**: Contract follows checks-effects-interactions pattern

## Troubleshooting

### Common Issues

1. **"SECRET_KEY not found"**: Copy `.env.example` to `.env` and set your key
2. **"Insufficient funds"**: Get testnet CSPR from the [faucet](https://testnet.cspr.live/tools/faucet)
3. **"Node connection failed"**: Check node address in `.env`
4. **"Odra not found"**: Install Odra CLI tools from [odra.dev](https://odra.dev)

### Getting Help

- Check [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment guide
- Visit [Odra Documentation](https://odra.dev) for framework help
- See [Casper Documentation](https://docs.casper.network) for network info

## License

MIT License