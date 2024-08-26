use clap::Parser;
use rcon_client::{AuthRequest, AuthResponse, RCONClient, RCONConfig, RCONError, RCONRequest};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long)]
    password: String,

    #[arg(short, long)]
    hostname: String,

    #[arg(short = 'p', long)]
    port: u16,
}

fn main() -> Result<(), RCONError> {
    // Parse command line arguments
    let args = Args::parse();

    // Create new RCON client
    let mut client = RCONClient::new(RCONConfig {
        url: format!("{}:{}", args.hostname, args.port),
        // Optional
        read_timeout: Some(13),
        write_timeout: Some(37),
    })?;

    // Auth request to RCON server (SERVERDATA_AUTH)
    let auth_result: AuthResponse = client.auth(AuthRequest::new(args.password))?;
    assert!(auth_result.is_success());

    // Execute command request to RCON server (SERVERDATA_EXECCOMMAND)
    let response = client.execute(RCONRequest::new("Hello, World!".to_string()))?;
    println!("Server response: {}", response.body);

    Ok(())
}
