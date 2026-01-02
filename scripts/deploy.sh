#!/bin/bash

# CasperLiquid Deployment Script
# This script automates the deployment process for the CasperLiquid contract

set -e  # Exit on any error

echo "🚀 CasperLiquid Deployment Script"
echo "=================================="

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ Error: .env file not found!"
    echo "Please copy .env.example to .env and configure your SECRET_KEY"
    exit 1
fi

# Source environment variables
source .env

# Validate required environment variables
if [ -z "$SECRET_KEY" ] || [ "$SECRET_KEY" = "your_secret_key_here" ]; then
    echo "❌ Error: Please set a valid SECRET_KEY in your .env file"
    echo "You can generate one using: casper-client keygen <path>"
    exit 1
fi

# Set default values if not provided
NODE_ADDRESS=${NODE_ADDRESS:-"http://3.143.158.19:7777"}
NETWORK_NAME=${NETWORK_NAME:-"casper-test"}
GAS_PRICE=${GAS_PRICE:-1}
TTL=${TTL:-"30m"}

echo "📋 Deployment Configuration:"
echo "   Node Address: $NODE_ADDRESS"
echo "   Network: $NETWORK_NAME"
echo "   Gas Price: $GAS_PRICE"
echo "   TTL: $TTL"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo "🔍 Checking prerequisites..."

if ! command_exists cargo; then
    echo "❌ Error: cargo not found. Please install Rust and Cargo"
    exit 1
fi

if ! command_exists casper-client; then
    echo "⚠️  Warning: casper-client not found. Some verification steps may not work"
fi

echo "✅ Prerequisites check complete"
echo ""

# Build the contract
echo "🔨 Building contract..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Error: Contract build failed"
    exit 1
fi

echo "✅ Contract built successfully"
echo ""

# Deploy using Odra
echo "🚀 Deploying contract to $NETWORK_NAME..."
echo "This may take a few minutes..."

# Check if odra command is available
if command_exists cargo-odra || cargo odra --help >/dev/null 2>&1; then
    cargo odra deploy --network "$NETWORK_NAME"
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ Contract deployed successfully!"
        echo ""
        echo "📝 Next Steps:"
        echo "   1. Save the contract hash from the output above"
        echo "   2. Update your frontend configuration with the contract hash"
        echo "   3. Test the contract functionality"
        echo "   4. Monitor the deployment on Casper testnet explorer"
        echo ""
        echo "🌐 Testnet Explorer: https://testnet.cspr.live/"
    else
        echo "❌ Deployment failed. Check the error messages above."
        exit 1
    fi
else
    echo "❌ Error: Odra CLI not found or not properly installed"
    echo "Please install Odra CLI tools and try again"
    echo "Visit: https://odra.dev for installation instructions"
    exit 1
fi

echo ""
echo "🎉 Deployment process complete!"