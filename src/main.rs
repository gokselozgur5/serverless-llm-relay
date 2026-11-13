use serverless_llm_relay::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    tracing::info!(
        port = config.port,
        model = %config.default_model,
        "serverless-llm-relay started"
    );
}
