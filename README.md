# CasperLiquid 🌊

A liquid staking protocol for the Casper Network built with the Odra Framework.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Casper Network](https://img.shields.io/badge/Casper-Network-red)](https://casper.network/)

## 🚀 Overview

CasperLiquid enables users to stake CSPR tokens and receive liquid staking tokens (stCSPR) at a 1:1 ratio, allowing them to earn staking rewards while maintaining liquidity for DeFi activities.

### Key Features

- **🔄 Liquid Staking**: Stake CSPR and receive stCSPR tokens (1:1 ratio)
- **⚡ Instant Unstaking**: Convert stCSPR back to CSPR anytime
- **🪙 CEP-18 Compliant**: Full compatibility with Casper wallets and DEXs
- **🌐 Web Interface**: User-friendly frontend with Casper Wallet integration
- **🔒 Security First**: Comprehensive testing and security measures
- **📱 Mobile Ready**: Responsive design for all devices

## 🏗️ Architecture

### Smart Contract
- **Framework**: Odra (Rust-based smart contracts for Casper)
- **Token Standard**: CEP-18 compliant stCSPR tokens
- **Security**: Reentrancy protection, input validation, atomic operations

### Frontend
- **Technology**: HTML/JavaScript with Casper JS SDK
- **Wallet**: Seamless Casper Wallet integration
- **Features**: Real-time balance updates, transaction status tracking

## 🛠️ Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Odra Framework](https://odra.dev/)
- [Casper Wallet](https://www.casperwallet.io/)
- Testnet CSPR tokens from [faucet](https://testnet.cspr.live/tools/faucet)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Anshulmehra001/CasperLiquid.git
   cd CasperLiquid
   ```

2. **Setup environment**
   ```bash
   cp .env.example .env
   # Edit .env and add your SECRET_KEY
   ```

3. **Build the project**
   ```bash
   cargo build --release
   ```

### Deployment

#### Option 1: Using Odra CLI
```bash
cargo odra deploy --network casper-test
```

#### Option 2: Using deployment scripts
```bash
# Windows PowerShell
.\scripts\deploy.ps1

# Unix/Linux/macOS
./scripts/deploy.sh
```

#### Option 3: Using deployment binary
```bash
cargo run -- deploy
```

## 📋 Contract Functions

### Core Staking Functions
```rust
pub fn stake(&mut self, amount: U256) -> Result<(), Error>
pub fn unstake(&mut self, amount: U256) -> Result<(), Error>
pub fn total_supply(&self) -> U256
```

### CEP-18 Token Functions
```rust
pub fn balance_of(&self, owner: Address) -> U256
pub fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), Error>
pub fn approve(&mut self, spender: Address, amount: U256) -> Result<(), Error>
pub fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) -> Result<(), Error>
pub fn allowance(&self, owner: Address, spender: Address) -> U256
```

### Metadata Functions
```rust
pub fn name(&self) -> String        // "Staked CSPR"
pub fn symbol(&self) -> String      // "stCSPR"
pub fn decimals(&self) -> u8        // 9 (matching CSPR)
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Frontend integration tests
cargo test --test frontend_integration_tests
```

### Test Coverage
- ✅ Unit tests for all contract functions
- ✅ Integration tests for end-to-end workflows
- ✅ Frontend integration tests
- ✅ Property-based testing for correctness validation

## 🌐 Frontend Usage

1. **Open the web interface**
   ```bash
   # Serve the frontend (after contract deployment)
   python -m http.server 8000
   # Open http://localhost:8000 in your browser
   ```

2. **Connect your Casper Wallet**
   - Click "Connect Wallet" button
   - Approve the connection in Casper Wallet

3. **Stake CSPR tokens**
   - Click "Stake 10 CSPR" to stake tokens
   - Confirm the transaction in your wallet
   - Receive stCSPR tokens instantly

4. **Unstake tokens**
   - Click "Unstake All" to convert stCSPR back to CSPR
   - Confirm the transaction
   - Receive CSPR tokens immediately

## ⚙️ Configuration

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

### Network Configuration
- **Target**: Casper Testnet
- **Node**: http://3.143.158.19:7777
- **Chain**: casper-test

## 🔐 Security Features

- **Input Validation**: All user inputs are validated before processing
- **Reentrancy Protection**: Follows checks-effects-interactions pattern
- **Atomic Operations**: State changes are atomic or not at all
- **Overflow Protection**: Safe arithmetic operations throughout
- **Access Control**: Proper permission handling for all functions

## 📁 Project Structure

```
casper-liquid/
├── src/
│   └── lib.rs              # Main contract implementation
├── bin/
│   └── main.rs             # Deployment binary
├── tests/
│   ├── integration_tests.rs         # Contract integration tests
│   └── frontend_integration_tests.rs # Frontend integration tests
├── scripts/
│   ├── deploy.sh           # Unix deployment script
│   └── deploy.ps1          # PowerShell deployment script
├── index.html              # Frontend web interface
├── Cargo.toml              # Rust dependencies
├── Odra.toml              # Odra configuration
├── .env.example           # Environment template
├── DEPLOYMENT.md          # Detailed deployment guide
└── README.md              # This file
```

## 🚨 Troubleshooting

### Common Issues

**"SECRET_KEY not found"**
```bash
cp .env.example .env
# Edit .env and add your secret key
```

**"Insufficient funds"**
- Get testnet CSPR from the [faucet](https://testnet.cspr.live/tools/faucet)

**"Node connection failed"**
- Check `NODE_ADDRESS` in your `.env` file
- Ensure the Casper testnet node is accessible

**"Odra not found"**
- Install Odra CLI tools: `cargo install odra-cli`

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📚 Resources

- [Odra Framework Documentation](https://odra.dev/)
- [Casper Network Documentation](https://docs.casper.network/)
- [CEP-18 Token Standard](https://github.com/casper-network/ceps/blob/master/text/0018-token-standard.md)
- [Casper Testnet Faucet](https://testnet.cspr.live/tools/faucet)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Odra Framework](https://odra.dev/)
- Powered by [Casper Network](https://casper.network/)
- Inspired by the DeFi ecosystem

---

**Ready to stake with liquidity? Try CasperLiquid today!** 🚀