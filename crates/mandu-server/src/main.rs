use clap::Parser;
use mandu_server::{create_app, AppState, ServerConfig};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mandu-server",
    author = "Lumen Pink <hi@lumen.pink>",
    version = mandu_core::VERSION,
    about = "Servidor HTTP REST e daemon backend para o ecossistema Mandu"
)]
struct Args {
    #[arg(short, long, env = "MANDU_PORT", default_value_t = 3000)]
    port: u16,

    #[arg(short, long, env = "MANDU_BIND_ADDR", default_value = "0.0.0.0")]
    bind: String,

    #[arg(short, long, env = "MANDU_VAULT_PATH", default_value = "notes")]
    vault: PathBuf,

    #[arg(
        long,
        env = "MANDU_AUTH_LOGOUT_URL",
        default_value = "https://auth.2lp.in/logout"
    )]
    auth_logout_url: String,

    #[arg(long, env = "MANDU_CORS_ORIGIN", default_value = "*")]
    cors_origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = ServerConfig {
        port: args.port,
        bind_addr: args.bind.clone(),
        vault_path: args.vault.clone(),
        auth_logout_url: args.auth_logout_url,
        cors_origin: args.cors_origin,
    };

    let state = AppState::new(config);
    let app = create_app(state);

    let addr: SocketAddr = format!("{}:{}", args.bind, args.port).parse()?;
    println!("mandu-server rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
