use mandu_server::{create_app, AppState, ServerConfig};

#[tokio::main]
async fn main() {
    let config = ServerConfig::default();
    let state = AppState::new(config);
    let _app = create_app(state);
    println!("mandu-server stub");
}
