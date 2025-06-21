use anyhow::Result;
use tracing_subscriber::{fmt, EnvFilter};

/// Initialize global tracing subscriber for applications.
pub fn init_tracing() -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive("scroll_core=info".parse()?);
    let format_env = std::env::var("SCROLL_TRACE_FORMAT")
        .unwrap_or_default()
        .to_lowercase();
    if format_env == "pretty" || cfg!(feature = "compact_tracing") {
        fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .without_time()
            .init();
    } else {
        fmt()
            .with_env_filter(filter)
            .json()
            .with_current_span(false)
            .without_time()
            .with_writer(std::io::stderr)
            .init();
    }
    Ok(())
}

/// Initialize tracing for unit tests. Subsequent calls are ignored.
pub fn init_tracing_for_test() -> Result<()> {
    if tracing::dispatcher::has_been_set() {
        return Ok(());
    }
    let filter = EnvFilter::from_default_env().add_directive("scroll_core=info".parse()?);
    let format_env = std::env::var("SCROLL_TRACE_FORMAT")
        .unwrap_or_default()
        .to_lowercase();
    if format_env == "pretty" || cfg!(feature = "compact_tracing") {
        let _ = fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .without_time()
            .try_init();
    } else {
        let _ = fmt()
            .with_env_filter(filter)
            .json()
            .with_current_span(false)
            .without_time()
            .with_writer(std::io::stderr)
            .try_init();
    }
    Ok(())
}
