use serverless_llm_relay::{gateway::create_app, Config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    tracing::info!(
        port = config.port,
        model = %config.default_model,
        "serverless-llm-relay started"
    );
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, create_app()).await.expect("serve");
}
