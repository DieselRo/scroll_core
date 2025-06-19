// src/telemetry/mod.rs

#[cfg(feature = "metrics")]
use metrics_exporter_prometheus::PrometheusBuilder;

#[cfg(feature = "metrics")]
/// Initializes the Prometheus metrics exporter on port 9898.
pub fn init() {
    let builder = PrometheusBuilder::new().with_http_listener(([0, 0, 0, 0], 9898));
    builder
        .install()
        .expect("failed to install Prometheus recorder");
}
