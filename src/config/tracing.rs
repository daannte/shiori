use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, prelude::*};

pub fn init() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let log_layer = tracing_subscriber::fmt::layer()
        .compact()
        .without_time()
        .with_filter(env_filter)
        .boxed();

    tracing_subscriber::registry().with(log_layer).init();
}
