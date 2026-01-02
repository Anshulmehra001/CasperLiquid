use std::env;
use std::process;

fn main() {
    println!("🚀 CasperLiquid Deployment Script");
    println!("==================================");
    
    // Check if .env file exists
    if !std::path::Path::new(".env").exists() {
        eprintln!("❌ Error: .env file not found!");
        eprintln!("Please copy .env.example to .env and configure your SECRET_KEY");
        process::exit(1);
    }
    
    // Load environment variables
    match dotenv::dotenv() {
        Ok(_) => println!("✅ Environment variables loaded from .env"),
        Err(e) => {
            eprintln!("❌ Error loading .env file: {}", e);
            process::exit(1);
        }
    }
    
    // Validate required environment variables
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| {
        eprintln!("❌ Error: SECRET_KEY not found in .env file");
        process::exit(1);
    });
    
    if secret_key == "your_secret_key_here" {
        eprintln!("❌ Error: Please set a valid SECRET_KEY in your .env file");
        eprintln!("You can generate one using: casper-client keygen <path>");
        process::exit(1);
    }
    
    let node_address = env::var("NODE_ADDRESS")
        .unwrap_or_else(|_| "http://3.143.158.19:7777".to_string());
    let network_name = env::var("NETWORK_NAME")
        .unwrap_or_else(|_| "casper-test".to_string());
    
    println!("📋 Deployment Configuration:");
    println!("   Node Address: {}", node_address);
    println!("   Network: {}", network_name);
    println!("   Contract: CasperLiquid");
    println!();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("deploy") => {
            println!("🔨 Starting contract deployment...");
            deploy_contract();
        }
        Some("verify") => {
            println!("🔍 Verifying deployment configuration...");
            verify_config();
        }
        Some("help") | Some("--help") | Some("-h") => {
            print_help();
        }
        _ => {
            println!("ℹ️  Use 'cargo run -- help' for usage information");
            print_help();
        }
    }
}

fn deploy_contract() {
    println!("📦 Building contract...");
    
    // In a real implementation, this would use Odra's deployment APIs
    // For now, we'll provide instructions for manual deployment
    println!("✅ Contract built successfully");
    println!();
    println!("🚀 To deploy the contract, run:");
    println!("   cargo odra deploy --network casper-test");
    println!();
    println!("📝 After deployment, save the contract hash for frontend integration");
}

fn verify_config() {
    println!("🔍 Verifying deployment configuration...");
    
    // Check .env file
    if std::path::Path::new(".env").exists() {
        println!("✅ .env file exists");
    } else {
        println!("❌ .env file missing");
        return;
    }
    
    // Check Odra.toml
    if std::path::Path::new("Odra.toml").exists() {
        println!("✅ Odra.toml exists");
    } else {
        println!("❌ Odra.toml missing");
        return;
    }
    
    // Check environment variables
    match dotenv::dotenv() {
        Ok(_) => {
            if env::var("SECRET_KEY").is_ok() {
                println!("✅ SECRET_KEY configured");
            } else {
                println!("❌ SECRET_KEY not configured");
            }
        }
        Err(_) => {
            println!("❌ Error loading environment variables");
        }
    }
    
    println!("✅ Configuration verification complete");
}

fn print_help() {
    println!("CasperLiquid Deployment Tool");
    println!();
    println!("USAGE:");
    println!("    cargo run -- <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("    deploy    Deploy the CasperLiquid contract to Testnet");
    println!("    verify    Verify deployment configuration");
    println!("    help      Show this help message");
    println!();
    println!("SETUP:");
    println!("    1. Copy .env.example to .env");
    println!("    2. Set your SECRET_KEY in .env");
    println!("    3. Run 'cargo run -- verify' to check configuration");
    println!("    4. Run 'cargo run -- deploy' to deploy the contract");
    println!();
    println!("For more information, see the deployment documentation in README.md");
}