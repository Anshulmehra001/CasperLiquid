# CasperLiquid Deployment Script (PowerShell)
# This script automates the deployment process for the CasperLiquid contract

param(
    [string]$Network = "casper-test",
    [switch]$Verify,
    [switch]$Help
)

function Write-Header {
    Write-Host "CasperLiquid Deployment Script" -ForegroundColor Cyan
    Write-Host "==============================" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Success {
    param([string]$Message)
    Write-Host "SUCCESS: $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "ERROR: $Message" -ForegroundColor Red
}

function Write-Warning {
    param([string]$Message)
    Write-Host "WARNING: $Message" -ForegroundColor Yellow
}

function Write-Info {
    param([string]$Message)
    Write-Host "INFO: $Message" -ForegroundColor Blue
}

function Show-Help {
    Write-Host "CasperLiquid Deployment Tool (PowerShell)" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "USAGE:" -ForegroundColor Yellow
    Write-Host "    .\scripts\deploy.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "OPTIONS:" -ForegroundColor Yellow
    Write-Host "    -Network name      Target network (default: casper-test)"
    Write-Host "    -Verify           Verify configuration only"
    Write-Host "    -Help             Show this help message"
    Write-Host ""
    Write-Host "EXAMPLES:" -ForegroundColor Yellow
    Write-Host "    .\scripts\deploy.ps1                    # Deploy to casper-test"
    Write-Host "    .\scripts\deploy.ps1 -Verify           # Verify configuration"
    Write-Host "    .\scripts\deploy.ps1 -Network mainnet  # Deploy to mainnet"
    Write-Host ""
    Write-Host "SETUP:" -ForegroundColor Yellow
    Write-Host "    1. Copy .env.example to .env"
    Write-Host "    2. Set your SECRET_KEY in .env"
    Write-Host "    3. Run with -Verify to check configuration"
    Write-Host "    4. Run without -Verify to deploy"
}

function Test-Prerequisites {
    Write-Info "Checking prerequisites..."
    
    # Check if cargo is available
    try {
        $null = Get-Command cargo -ErrorAction Stop
        Write-Success "Cargo found"
        return $true
    }
    catch {
        Write-Error "Cargo not found. Please install Rust and Cargo"
        return $false
    }
}

function Test-Configuration {
    Write-Info "Verifying deployment configuration..."
    
    # Check .env file
    if (Test-Path ".env") {
        Write-Success ".env file exists"
    }
    else {
        Write-Error ".env file missing. Copy .env.example to .env and configure it."
        return $false
    }
    
    # Check Odra.toml
    if (Test-Path "Odra.toml") {
        Write-Success "Odra.toml exists"
    }
    else {
        Write-Error "Odra.toml missing"
        return $false
    }
    
    # Load and check environment variables
    try {
        Get-Content ".env" | ForEach-Object {
            if ($_ -match "^([^#][^=]+)=(.*)$") {
                [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
            }
        }
        
        $secretKey = [Environment]::GetEnvironmentVariable("SECRET_KEY", "Process")
        if ($secretKey -and $secretKey -ne "your_secret_key_here") {
            Write-Success "SECRET_KEY configured"
        }
        else {
            Write-Error "SECRET_KEY not properly configured in .env"
            return $false
        }
    }
    catch {
        Write-Error "Error loading environment variables from .env"
        return $false
    }
    
    Write-Success "Configuration verification complete"
    return $true
}

function Start-Deployment {
    param([string]$TargetNetwork)
    
    Write-Info "Building contract..."
    
    # Build the contract
    try {
        & cargo build --release
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Contract build failed"
            return $false
        }
    }
    catch {
        Write-Error "Failed to run cargo build"
        return $false
    }
    
    Write-Success "Contract built successfully"
    Write-Host ""
    
    # Deploy using Odra
    Write-Info "Deploying contract to $TargetNetwork..."
    Write-Warning "This may take a few minutes..."
    
    try {
        & cargo odra deploy --network $TargetNetwork
        if ($LASTEXITCODE -eq 0) {
            Write-Host ""
            Write-Success "Contract deployed successfully!"
            Write-Host ""
            Write-Host "Next Steps:" -ForegroundColor Yellow
            Write-Host "   1. Save the contract hash from the output above"
            Write-Host "   2. Update your frontend configuration with the contract hash"
            Write-Host "   3. Test the contract functionality"
            Write-Host "   4. Monitor the deployment on Casper testnet explorer"
            Write-Host ""
            Write-Host "Testnet Explorer: https://testnet.cspr.live/" -ForegroundColor Cyan
            return $true
        }
        else {
            Write-Error "Deployment failed. Check the error messages above."
            return $false
        }
    }
    catch {
        Write-Error "Odra CLI not found or not properly installed"
        Write-Warning "Please install Odra CLI tools and try again"
        Write-Info "Visit: https://odra.dev for installation instructions"
        return $false
    }
}

# Main execution
Write-Header

if ($Help) {
    Show-Help
    exit 0
}

# Check prerequisites
if (-not (Test-Prerequisites)) {
    exit 1
}

Write-Host ""

# Verify configuration
if (-not (Test-Configuration)) {
    exit 1
}

if ($Verify) {
    Write-Host ""
    Write-Success "Configuration verification complete. Ready for deployment!"
    exit 0
}

Write-Host ""

# Load environment variables
Get-Content ".env" | ForEach-Object {
    if ($_ -match "^([^#][^=]+)=(.*)$") {
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}

$nodeAddress = [Environment]::GetEnvironmentVariable("NODE_ADDRESS", "Process")
if (-not $nodeAddress) { $nodeAddress = "http://3.143.158.19:7777" }

$gasPrice = [Environment]::GetEnvironmentVariable("GAS_PRICE", "Process")
if (-not $gasPrice) { $gasPrice = "1" }

$ttl = [Environment]::GetEnvironmentVariable("TTL", "Process")
if (-not $ttl) { $ttl = "30m" }

Write-Host "Deployment Configuration:" -ForegroundColor Blue
Write-Host "   Node Address: $nodeAddress"
Write-Host "   Network: $Network"
Write-Host "   Gas Price: $gasPrice"
Write-Host "   TTL: $ttl"
Write-Host ""

# Start deployment
if (Start-Deployment -TargetNetwork $Network) {
    Write-Host ""
    Write-Host "Deployment process complete!" -ForegroundColor Green
    exit 0
}
else {
    exit 1
}