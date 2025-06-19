use crate::errors::MetricError;

pub fn clamp_finite(v: f64) -> Result<f64, MetricError> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(MetricError::NonFinite)
    }
}
