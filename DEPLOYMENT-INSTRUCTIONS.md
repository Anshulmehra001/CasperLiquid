# Quick Deployment for Hackathon Submission

## To get your Casper Testnet Contract Address:

### Step 1: Setup Environment
```bash
cd casper-liquid
cp .env.example .env
```

### Step 2: Add your secret key to .env
```bash
# Edit .env file and add:
SECRET_KEY=your_casper_wallet_secret_key_here
```

### Step 3: Deploy to Casper Testnet
```bash
cargo odra deploy --network casper-test
```

### Step 4: Copy the contract hash from deployment output
The deployment will output something like:
```
Contract deployed successfully!
Contract Hash: hash-1234567890abcdef...
```

### Step 5: Update your submission with the contract hash

## Alternative: Use placeholder for now
If you can't deploy immediately, you can submit with:
"Contract ready for deployment - will provide hash upon request"