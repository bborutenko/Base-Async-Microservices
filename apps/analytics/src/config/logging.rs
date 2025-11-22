use tracing_subscriber::{EnvFilter, fmt, prelude::*, registry};

pub fn setup_logging() {
    let console_layer = fmt::layer()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_span_events(fmt::format::FmtSpan::NEW | fmt::format::FmtSpan::CLOSE)
        .compact();

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    registry().with(console_layer).with(filter_layer).init();
}
