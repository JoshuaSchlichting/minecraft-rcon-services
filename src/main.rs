use clap::Parser;
use rcon_client::{AuthRequest, AuthResponse, RCONClient, RCONConfig, RCONError, RCONRequest};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    password: String,

    #[arg(long)]
    hostname: String,

    #[arg(short = 'p', long)]
    port: u16,
}

fn main() -> Result<(), RCONError> {
    let args = Args::parse();

    let mut client = RCONClient::new(RCONConfig {
        url: format!("{}:{}", args.hostname, args.port),
        // Optional
        read_timeout: Some(13),
        write_timeout: Some(37),
    })?;

    let auth_result: AuthResponse = client.auth(AuthRequest::new(args.password))?;
    assert!(auth_result.is_success());

    // Execute command request to RCON server (SERVERDATA_EXECCOMMAND)
    let response = client.execute(RCONRequest::new("Hello, World!".to_string()))?;
    println!("Server response: {}", response.body);

    Ok(())
}
