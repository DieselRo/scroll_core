use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetricError {
    #[error("non-finite metric value")]
    NonFinite,
}
