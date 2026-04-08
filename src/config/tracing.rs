use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

pub fn init() {
    let env_filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::INFO.into())
        .add_directive(
            "shiori=trace"
                .parse()
                .expect("Invalid tracing directive for shiori"),
        )
        .add_directive(
            "html5ever::tree_builder=off"
                .parse()
                .expect("Invalid tracing directive for html5ever"),
        );

    let log_layer = tracing_subscriber::fmt::layer().compact().without_time();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(log_layer)
        .init();

    tracing::info!("Tracing initialized");
}
